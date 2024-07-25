use crate::naive_date_time::NaiveDateTimeClassification;
use crate::Classify;
use chrono::{DateTime, Datelike, TimeZone, Utc};

impl<Tz: TimeZone> Classify for DateTime<Tz> {
    type Output = NaiveDateTimeClassification;

    fn classify(&self) -> Self::Output {
        let epoch = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap();

        if self == &epoch {
            NaiveDateTimeClassification::Default
        } else if self < &epoch || self.year() >= 2050 {
            NaiveDateTimeClassification::ProbablyError
        } else {
            NaiveDateTimeClassification::Normal
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Days, Utc};
    use std::ops::{Add, Sub};

    fn default() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap()
    }

    #[test]
    fn test_naive_date_classify() {
        // Test the epoch date (Jan 1st, 1970)
        let epoch = default();
        assert_eq!(epoch.classify(), NaiveDateTimeClassification::Default);

        // Test a date after the epoch
        let future_date = Utc::now();
        assert_eq!(future_date.classify(), NaiveDateTimeClassification::Normal);

        // Test a date before the epoch
        let past_date = default().sub(Days::new(1));
        assert_eq!(
            past_date.classify(),
            NaiveDateTimeClassification::ProbablyError
        );

        // Test the day after the epoch
        let day_after_epoch = default().add(Days::new(1));
        assert_eq!(
            day_after_epoch.classify(),
            NaiveDateTimeClassification::Normal
        );

        // Test a far future date
        let far_future = default().add(Days::new(365 * 1000));
        assert_eq!(
            far_future.classify(),
            NaiveDateTimeClassification::ProbablyError
        );

        // Test a far past date
        let far_past = default().sub(Days::new(365 * 1000));
        assert_eq!(
            far_past.classify(),
            NaiveDateTimeClassification::ProbablyError
        );
    }
}
