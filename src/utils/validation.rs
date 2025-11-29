use validator::ValidationError;
use crate::models::DateRange;

// Validation function for DateRange to check that start is before end
pub fn validate_date_range(range: &DateRange) -> Result<(), ValidationError> {
    if range.start > range.end {
        return Err(ValidationError::new("start_after_end"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    
    #[test]
    fn test_validate_date_range_valid() {
        let range = DateRange::new(
            Some(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()),
            Some(NaiveDate::from_ymd_opt(2023, 12, 31).unwrap()),
        );
        assert!(validate_date_range(&range).is_ok());
    }
    
    #[test]
    fn test_validate_date_range_same_dates() {
        let date = NaiveDate::from_ymd_opt(2023, 6, 15).unwrap();
        let range = DateRange::new(Some(date), Some(date));
        assert!(validate_date_range(&range).is_ok());
    }
    
    #[test]
    fn test_validate_date_range_invalid() {
        let range = DateRange::new(
            Some(NaiveDate::from_ymd_opt(2023, 12, 31).unwrap()),
            Some(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()),
        );
        let result = validate_date_range(&range);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, "start_after_end");
    }
}
