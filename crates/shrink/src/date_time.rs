use crate::Classify;
use chrono::{Datelike, NaiveDateTime};

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum NaiveDateTimeClassification {
    /// Any date prior to the epoch, Jan 1st, 1970
    /// or after Jan 1st, 2050
    ProbablyError,
    /// The default date, epoch Jan 1st, 1970
    Default,
    /// Any date other than the epoch
    Normal,
}

impl Classify for NaiveDateTime {
    type Output = NaiveDateTimeClassification;

    fn classify(&self) -> Self::Output {
        let epoch = NaiveDateTime::default();

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
    use chrono::{Days, NaiveDateTime, Utc};
    use std::ops::{Add, Sub};

    #[test]
    fn test_naive_date_classify() {
        // Test the epoch date (Jan 1st, 1970)
        let epoch = NaiveDateTime::default();
        assert_eq!(epoch.classify(), NaiveDateTimeClassification::Default);

        // Test a date after the epoch
        let future_date = Utc::now().naive_utc();
        assert_eq!(future_date.classify(), NaiveDateTimeClassification::Normal);

        // Test a date before the epoch
        let past_date = NaiveDateTime::default().sub(Days::new(1));
        assert_eq!(
            past_date.classify(),
            NaiveDateTimeClassification::ProbablyError
        );

        // Test the day after the epoch
        let day_after_epoch = NaiveDateTime::default().add(Days::new(1));
        assert_eq!(
            day_after_epoch.classify(),
            NaiveDateTimeClassification::Normal
        );

        // Test a far future date
        let far_future = NaiveDateTime::default().add(Days::new(365 * 1000));
        assert_eq!(
            far_future.classify(),
            NaiveDateTimeClassification::ProbablyError
        );

        // Test a far past date
        let far_past = NaiveDateTime::default().sub(Days::new(365 * 1000));
        assert_eq!(
            far_past.classify(),
            NaiveDateTimeClassification::ProbablyError
        );
    }
}
