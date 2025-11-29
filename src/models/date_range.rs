use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, FromRow, Validate)]
pub struct DateRange {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

impl DateRange {
    const MIN_DATE: &'static str = "0001-01-01";
    const MAX_DATE: &'static str = "9999-12-31";
    
    pub fn new(start: Option<NaiveDate>, end: Option<NaiveDate>) -> Self {
        let min_date = NaiveDate::parse_from_str(Self::MIN_DATE, "%Y-%m-%d").unwrap();
        let max_date = NaiveDate::parse_from_str(Self::MAX_DATE, "%Y-%m-%d").unwrap();
        
        Self {
            start: start.unwrap_or(min_date),
            end: end.unwrap_or(max_date),
        }
    }
    
    pub fn from_string(start: Option<&str>, end: Option<&str>) -> Result<Self, chrono::ParseError> {
        let min_date = NaiveDate::parse_from_str(Self::MIN_DATE, "%Y-%m-%d").unwrap();
        let max_date = NaiveDate::parse_from_str(Self::MAX_DATE, "%Y-%m-%d").unwrap();
        
        let start_date = match start {
            Some(s) if !s.is_empty() => NaiveDate::parse_from_str(s, "%Y-%m-%d")?,
            _ => min_date,
        };
        
        let end_date = match end {
            Some(e) if !e.is_empty() => NaiveDate::parse_from_str(e, "%Y-%m-%d")?,
            _ => max_date,
        };
        
        Ok(Self {
            start: start_date,
            end: end_date,
        })
    }
    
    pub fn contains_date(&self, date: NaiveDate) -> bool {
        date >= self.start && date <= self.end
    }
    
    pub fn contains_date_str(&self, date_str: &str) -> Result<bool, chrono::ParseError> {
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
        Ok(self.contains_date(date))
    }
    
    pub fn is_valid(&self) -> bool {
        self.start <= self.end
    }
}

impl Default for DateRange {
    fn default() -> Self {
        let today = Utc::now().naive_utc().date();
        Self {
            start: today,
            end: today,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_date_range_new_with_values() {
        let start = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
        let range = DateRange::new(Some(start), Some(end));
        assert_eq!(range.start, start);
        assert_eq!(range.end, end);
    }
    
    #[test]
    fn test_date_range_new_with_defaults() {
        let range = DateRange::new(None, None);
        assert_eq!(range.start, NaiveDate::parse_from_str("0001-01-01", "%Y-%m-%d").unwrap());
        assert_eq!(range.end, NaiveDate::parse_from_str("9999-12-31", "%Y-%m-%d").unwrap());
    }
    
    #[test]
    fn test_date_range_from_string_success() {
        let range = DateRange::from_string(Some("2023-01-01"), Some("2023-12-31")).unwrap();
        assert_eq!(range.start, NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
        assert_eq!(range.end, NaiveDate::from_ymd_opt(2023, 12, 31).unwrap());
    }
    
    #[test]
    fn test_date_range_from_string_with_empty() {
        let range = DateRange::from_string(Some(""), Some("")).unwrap();
        assert_eq!(range.start, NaiveDate::parse_from_str("0001-01-01", "%Y-%m-%d").unwrap());
        assert_eq!(range.end, NaiveDate::parse_from_str("9999-12-31", "%Y-%m-%d").unwrap());
    }
    
    #[test]
    fn test_date_range_from_string_invalid_format() {
        let result = DateRange::from_string(Some("invalid"), Some("2023-12-31"));
        assert!(result.is_err());
    }
    
    #[test]
    fn test_contains_date_within_range() {
        let start = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
        let range = DateRange::new(Some(start), Some(end));
        let test_date = NaiveDate::from_ymd_opt(2023, 6, 15).unwrap();
        assert!(range.contains_date(test_date));
    }
    
    #[test]
    fn test_contains_date_outside_range() {
        let start = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
        let range = DateRange::new(Some(start), Some(end));
        let test_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        assert!(!range.contains_date(test_date));
    }
    
    #[test]
    fn test_contains_date_on_boundary() {
        let start = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
        let range = DateRange::new(Some(start), Some(end));
        assert!(range.contains_date(start));
        assert!(range.contains_date(end));
    }
    
    #[test]
    fn test_contains_date_str_success() {
        let start = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
        let range = DateRange::new(Some(start), Some(end));
        assert!(range.contains_date_str("2023-06-15").unwrap());
    }
    
    #[test]
    fn test_contains_date_str_invalid_format() {
        let range = DateRange::default();
        assert!(range.contains_date_str("invalid-date").is_err());
    }
    
    #[test]
    fn test_is_valid_true() {
        let start = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
        let range = DateRange::new(Some(start), Some(end));
        assert!(range.is_valid());
    }
    
    #[test]
    fn test_is_valid_false() {
        let start = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
        let end = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let range = DateRange { start, end };
        assert!(!range.is_valid());
    }
}