use validator::ValidationError;
use crate::models::DateRange;

// Validation function for DateRange to check that start is before end
pub fn validate_date_range(range: &DateRange) -> Result<(), ValidationError> {
    if range.start > range.end {
        return Err(ValidationError::new("start_after_end"));
    }
    Ok(())
}
