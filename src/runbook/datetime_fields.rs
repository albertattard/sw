//! Semantic field reconciliation before datetime conversion.
use jiff::{
    civil::{Date, DateTime, Time},
    fmt::strtime::{BrokenDownTime, Meridiem},
    tz::Offset,
};

/// Chrono stores full years, centuries, and two-digit years separately.
/// Jiff's parser sets the full year for each, so resolve them before attaching
/// a year to the parsed date. Keep repeated-field errors distinct from the
/// historical time-only fallback for conflicting *different* components.
#[derive(Clone, Copy, Default)]
pub(super) struct YearComponents {
    full: Option<i16>,
    century: Option<i16>,
    two_digit: Option<i16>,
}

impl YearComponents {
    pub(super) fn set_full(&mut self, value: i16) -> Result<(), String> {
        Self::set(&mut self.full, value)
    }

    pub(super) fn set_century(&mut self, value: i16) -> Result<(), String> {
        Self::set(&mut self.century, value)
    }

    pub(super) fn set_two_digit(&mut self, value: i16) -> Result<(), String> {
        Self::set(&mut self.two_digit, value)
    }

    fn set(field: &mut Option<i16>, value: i16) -> Result<(), String> {
        if field.is_some_and(|previous| previous != value) {
            return Err("conflicting repeated year component".into());
        }
        *field = Some(value);
        Ok(())
    }

    fn matches_filled(self, candidate: i16) -> bool {
        self.full.is_none_or(|year| year == candidate)
            && (self.century.is_none() && self.two_digit.is_none() || candidate >= 0)
            && self
                .century
                .is_none_or(|century| century == candidate / 100)
            && self.two_digit.is_none_or(|year| year == candidate % 100)
    }

    fn resolve(self) -> Result<Option<i16>, ()> {
        if let Some(full) = self.full {
            if (self.century.is_some() || self.two_digit.is_some())
                && (full < 0
                    || self.century.is_some_and(|c| c != full / 100)
                    || self.two_digit.is_some_and(|y| y != full % 100))
            {
                return Err(());
            }
            return Ok(Some(full));
        }
        match (self.century, self.two_digit) {
            (Some(century), Some(year)) => Ok(Some(century * 100 + year)),
            (None, Some(year)) => Ok(Some(year + if year < 70 { 2000 } else { 1900 })),
            (None, None) => Ok(None),
            (Some(_), None) => Err(()),
        }
    }
}

/// A 24-hour value supplies both constraints; a 12-hour value supplies the
/// remainder, and AM/PM supplies the half-day. Contradictions are parse errors
/// in Chrono, even when an epoch could otherwise provide a complete datetime.
#[derive(Default)]
pub(super) struct HourComponents {
    half_day: Option<i8>,
    remainder: Option<i8>,
}

impl HourComponents {
    pub(super) fn record(&mut self, kind: char, piece: &BrokenDownTime) -> Result<(), String> {
        match kind {
            'H' | 'k' => {
                let hour = piece.hour().expect("hour directive sets an hour");
                Self::set(&mut self.half_day, hour / 12)?;
                Self::set(&mut self.remainder, hour % 12)
            }
            'I' | 'l' => Self::set(
                &mut self.remainder,
                piece.hour().expect("hour directive sets an hour") % 12,
            ),
            'p' | 'P' => Self::set(
                &mut self.half_day,
                match piece.meridiem().expect("meridiem directive sets AM/PM") {
                    Meridiem::AM => 0,
                    Meridiem::PM => 1,
                },
            ),
            _ => unreachable!(),
        }
    }

    fn set(field: &mut Option<i8>, value: i8) -> Result<(), String> {
        if field.is_some_and(|previous| previous != value) {
            return Err("conflicting hour or meridiem fields".into());
        }
        *field = Some(value);
        Ok(())
    }

    fn complete(&self) -> Option<i8> {
        Some(self.half_day? * 12 + self.remainder?)
    }

    fn matches(&self, hour: i8) -> bool {
        self.half_day.is_none_or(|half| half == hour / 12)
            && self
                .remainder
                .is_none_or(|remainder| remainder == hour % 12)
    }
}

/// Keeps independent constraints available through full-date conversion, epoch
/// completion, and time-only fallback. The underlying parser is used for field
/// syntax and calendar arithmetic, never to select which fields take priority.
pub(crate) struct ParsedDatetime {
    fields: BrokenDownTime,
    year: YearComponents,
    iso_year: YearComponents,
    hours: HourComponents,
    quarter: Option<i8>,
}

impl ParsedDatetime {
    pub(super) fn new(
        mut fields: BrokenDownTime,
        year: YearComponents,
        iso_year: YearComponents,
        hours: HourComponents,
        quarter: Option<i8>,
    ) -> Result<Self, String> {
        let (resolved_year, resolved_iso_year) = match (year.resolve(), iso_year.resolve()) {
            (Ok(year), Ok(iso_year)) => (year, iso_year),
            _ => (None, None),
        };
        fields.set_year(resolved_year).map_err(|e| e.to_string())?;
        fields
            .set_iso_week_year(resolved_iso_year)
            .map_err(|e| e.to_string())?;
        fields
            .set_hour(hours.complete())
            .map_err(|e| e.to_string())?;
        Ok(Self {
            fields,
            year,
            iso_year,
            hours,
            quarter,
        })
    }

    pub(crate) fn offset(&self) -> Option<Offset> {
        self.fields.offset()
    }

    fn to_time(&self) -> Result<Time, String> {
        let hour = self
            .hours
            .complete()
            .ok_or("hour and meridiem are incomplete")?;
        let minute = self.fields.minute().ok_or("minute is required")?;
        if self.fields.subsec_nanosecond().is_some() && self.fields.second().is_none() {
            return Err("seconds are required with fractional seconds".into());
        }
        Time::new(
            hour,
            minute,
            self.fields.second().unwrap_or(0),
            self.fields.subsec_nanosecond().unwrap_or(0),
        )
        .map_err(|e| e.to_string())
    }

    fn to_date(&self) -> Result<Date, String> {
        let year = self
            .year
            .resolve()
            .map_err(|_| "inconsistent or incomplete year")?;
        let iso_year = self
            .iso_year
            .resolve()
            .map_err(|_| "inconsistent ISO year")?;
        // A complete Gregorian date determines the ISO century. Resolve it
        // without Jiff's already-pivoted ISO year, then validate every ISO
        // component against the resulting date. If ISO fields establish the
        // date, retain the pivoted year for that conversion.
        let gregorian = match (year, self.fields.month(), self.fields.day()) {
            (Some(year), Some(month), Some(day)) => Date::new(year, month, day).ok(),
            (Some(year), _, _) => self.fields.day_of_year().and_then(|ordinal| {
                Date::new(year, 1, 1).ok().and_then(|date| {
                    date.checked_add(jiff::Span::new().days(i64::from(ordinal - 1)))
                        .ok()
                })
            }),
            _ => None,
        };
        // A complete ISO week date determines its Gregorian year. Only use
        // the Gregorian pivot to construct a date when Gregorian fields (or a
        // Sunday/Monday week date) actually establish it.
        let non_iso_week = year.is_some()
            && self.fields.weekday().is_some()
            && (self.fields.sunday_based_week().is_some()
                || self.fields.monday_based_week().is_some());
        let week_date = if non_iso_week && gregorian.is_none() {
            let mut week_fields = BrokenDownTime::default();
            week_fields.set_year(year).map_err(|e| e.to_string())?;
            week_fields.set_weekday(self.fields.weekday());
            if let Some(week) = self.fields.sunday_based_week() {
                week_fields
                    .set_sunday_based_week(Some(week))
                    .map_err(|e| e.to_string())?;
            } else {
                week_fields
                    .set_monday_based_week(self.fields.monday_based_week())
                    .map_err(|e| e.to_string())?;
            }
            Some(week_fields.to_date().map_err(|e| e.to_string())?)
        } else {
            None
        };
        let iso_date = match (iso_year, self.fields.iso_week(), self.fields.weekday()) {
            (Some(year), Some(week), Some(weekday)) if !non_iso_week => {
                jiff::civil::ISOWeekDate::new(year, week, weekday)
                    .ok()
                    .map(|value| value.date())
            }
            _ => None,
        };
        let (date, year_matches) = match (gregorian.or(week_date), iso_date) {
            (Some(date), _) => (date, year.is_none_or(|year| year == date.year())),
            (None, Some(date)) => (date, self.year.matches_filled(date.year())),
            (None, None) => {
                let date = self.fields.to_date().map_err(|e| e.to_string())?;
                (date, year.is_none_or(|year| year == date.year()))
            }
        };
        if !self.matches_date(date, year_matches) {
            return Err("inconsistent calendar fields".into());
        }
        Ok(date)
    }

    fn matches_date(&self, date: Date, year_matches: bool) -> bool {
        let iso = date.iso_week_date();
        let iso_year_matches = self.iso_year.matches_filled(iso.year());
        let sunday_week =
            (date.day_of_year() + 6 - i16::from(date.weekday().to_sunday_zero_offset())) / 7;
        let monday_week =
            (date.day_of_year() + 6 - i16::from(date.weekday().to_monday_zero_offset())) / 7;
        year_matches
            && iso_year_matches
            && self
                .fields
                .month()
                .is_none_or(|month| month == date.month())
            && self.fields.day().is_none_or(|day| day == date.day())
            && self
                .fields
                .day_of_year()
                .is_none_or(|day| day == date.day_of_year())
            && self
                .fields
                .weekday()
                .is_none_or(|weekday| weekday == date.weekday())
            && self.fields.iso_week().is_none_or(|week| week == iso.week())
            && self
                .fields
                .sunday_based_week()
                .is_none_or(|week| i16::from(week) == sunday_week)
            && self
                .fields
                .monday_based_week()
                .is_none_or(|week| i16::from(week) == monday_week)
            && self
                .quarter
                .is_none_or(|quarter| quarter == (date.month() - 1) / 3 + 1)
    }

    pub(crate) fn to_datetime(&self) -> Result<DateTime, String> {
        // A fully supplied date/time wins only after its epoch is checked.
        if let (Ok(date), Ok(time)) = (self.to_date(), self.to_time()) {
            let local = date.to_datetime(time);
            if let Some(epoch) = self.fields.timestamp()
                && Offset::UTC
                    .to_timestamp(local)
                    .map_err(|e| e.to_string())?
                    .as_second()
                    != epoch.as_second()
            {
                return Err("epoch conflicts with calendar or time fields".into());
            }
            return Ok(local);
        }

        // When information is missing, Chrono fills it from the epoch and then
        // checks every supplied constraint. Parsed offsets are ignored for this
        // naive clock conversion, as they were before the dependency migration.
        let epoch = self
            .fields
            .timestamp()
            .ok_or("date/time fields are incomplete or inconsistent")?;
        let fraction = jiff::SignedDuration::new(0, self.fields.subsec_nanosecond().unwrap_or(0));
        let epoch = epoch.checked_add(fraction).map_err(|e| e.to_string())?;
        let local = Offset::UTC.to_datetime(epoch);
        if !self.matches_date(local.date(), self.year.matches_filled(local.year()))
            || !self.hours.matches(local.hour())
            || self
                .fields
                .minute()
                .is_some_and(|minute| minute != local.minute())
            || self
                .fields
                .second()
                .is_some_and(|second| second != local.second())
        {
            return Err("epoch conflicts with supplied datetime fields".into());
        }
        Ok(local)
    }

    pub(crate) fn to_local_datetime(&self, base: DateTime) -> Result<DateTime, String> {
        self.to_datetime()
            .or_else(|_| self.to_time().map(|time| base.date().to_datetime(time)))
    }
}
