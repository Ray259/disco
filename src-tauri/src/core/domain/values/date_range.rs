use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateRange {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

impl DateRange {
    pub fn new(start: NaiveDate, end: NaiveDate) -> Self {
        Self { start, end }
    }

    pub fn from_optional(start: Option<NaiveDate>, end: Option<NaiveDate>) -> Option<Self> {
        match (start, end) {
            (Some(s), Some(e)) => Some(Self::new(s, e)),
            _ => None,
        }
    }

    pub fn duration_days(&self) -> i64 {
        (self.end - self.start).num_days()
    }

    pub fn contains(&self, date: NaiveDate) -> bool {
        date >= self.start && date <= self.end
    }
}