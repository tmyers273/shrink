use crate::Classify;
use chrono::{Datelike, NaiveDate};

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum NaiveDateClassification {
    /// Any date prior to the epoch, Jan 1st, 1970
    /// or after Jan 1st, 2050
    ProbablyError,
    /// The default date, epoch Jan 1st, 1970
    Default,
    /// Any date other than the epoch
    Normal,
}

impl Classify for NaiveDate {
    type Output = NaiveDateClassification;

    fn classify(&self) -> Self::Output {
        let epoch = NaiveDate::default();

        if self == &epoch {
            NaiveDateClassification::Default
        } else if self < &epoch || self.year() >= 2050 {
            NaiveDateClassification::ProbablyError
        } else {
            NaiveDateClassification::Normal
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_naive_date_classify() {
        // Test the epoch date (Jan 1st, 1970)
        let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
        assert_eq!(epoch.classify(), NaiveDateClassification::Default);

        // Test a date after the epoch
        let future_date = NaiveDate::from_ymd_opt(2023, 7, 26).unwrap();
        assert_eq!(future_date.classify(), NaiveDateClassification::Normal);

        // Test a date before the epoch
        let past_date = NaiveDate::from_ymd_opt(1969, 12, 31).unwrap();
        assert_eq!(past_date.classify(), NaiveDateClassification::ProbablyError);

        // Test the day after the epoch
        let day_after_epoch = NaiveDate::from_ymd_opt(1970, 1, 2).unwrap();
        assert_eq!(day_after_epoch.classify(), NaiveDateClassification::Normal);

        // Test a far future date
        let far_future = NaiveDate::from_ymd_opt(9999, 12, 31).unwrap();
        assert_eq!(
            far_future.classify(),
            NaiveDateClassification::ProbablyError
        );

        // Test a far past date
        let far_past = NaiveDate::from_ymd_opt(1, 1, 1).unwrap();
        assert_eq!(far_past.classify(), NaiveDateClassification::ProbablyError);
    }
}
