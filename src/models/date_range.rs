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