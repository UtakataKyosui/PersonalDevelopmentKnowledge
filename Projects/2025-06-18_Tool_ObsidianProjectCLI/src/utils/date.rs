use chrono::{DateTime, Utc};

pub fn format_relative_time(datetime: &DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(*datetime);

    if duration.num_days() == 0 {
        if duration.num_hours() == 0 {
            if duration.num_minutes() == 0 {
                "Just now".to_string()
            } else {
                format!("{} min ago", duration.num_minutes())
            }
        } else {
            format!("{} hours ago", duration.num_hours())
        }
    } else if duration.num_days() == 1 {
        "1 day ago".to_string()
    } else if duration.num_days() < 7 {
        format!("{} days ago", duration.num_days())
    } else if duration.num_weeks() == 1 {
        "1 week ago".to_string()
    } else if duration.num_weeks() < 4 {
        format!("{} weeks ago", duration.num_weeks())
    } else {
        datetime.format("%Y-%m-%d").to_string()
    }
}

pub fn format_duration_days(days: i64) -> String {
    if days == 0 {
        "Today".to_string()
    } else if days == 1 {
        "1 day".to_string()
    } else if days < 7 {
        format!("{} days", days)
    } else if days < 30 {
        let weeks = days / 7;
        if weeks == 1 {
            "1 week".to_string()
        } else {
            format!("{} weeks", weeks)
        }
    } else {
        let months = days / 30;
        if months == 1 {
            "1 month".to_string()
        } else {
            format!("{} months", months)
        }
    }
}

pub fn parse_date_string(date_str: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    // Try different date formats
    let formats = [
        "%Y-%m-%d",
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%dT%H:%M:%SZ",
        "%Y-%m-%dT%H:%M:%S%.fZ",
    ];

    for format in &formats {
        if let Ok(naive_datetime) = chrono::NaiveDateTime::parse_from_str(date_str, format) {
            return Ok(DateTime::from_utc(naive_datetime, Utc));
        }
        if let Ok(naive_date) = chrono::NaiveDate::parse_from_str(date_str, format) {
            return Ok(DateTime::from_utc(naive_date.and_hms(0, 0, 0), Utc));
        }
    }

    // Try RFC3339 format
    date_str.parse::<DateTime<Utc>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_format_relative_time() {
        let now = Utc::now();
        
        // Test various time differences
        let five_minutes_ago = now - Duration::minutes(5);
        assert_eq!(format_relative_time(&five_minutes_ago), "5 min ago");
        
        let two_hours_ago = now - Duration::hours(2);
        assert_eq!(format_relative_time(&two_hours_ago), "2 hours ago");
        
        let yesterday = now - Duration::days(1);
        assert_eq!(format_relative_time(&yesterday), "1 day ago");
        
        let three_days_ago = now - Duration::days(3);
        assert_eq!(format_relative_time(&three_days_ago), "3 days ago");
    }

    #[test]
    fn test_format_duration_days() {
        assert_eq!(format_duration_days(0), "Today");
        assert_eq!(format_duration_days(1), "1 day");
        assert_eq!(format_duration_days(5), "5 days");
        assert_eq!(format_duration_days(7), "1 week");
        assert_eq!(format_duration_days(14), "2 weeks");
        assert_eq!(format_duration_days(30), "1 month");
        assert_eq!(format_duration_days(60), "2 months");
    }

    #[test]
    fn test_parse_date_string() {
        let date_str = "2025-06-18";
        let parsed = parse_date_string(date_str).unwrap();
        assert_eq!(parsed.format("%Y-%m-%d").to_string(), "2025-06-18");
        
        let datetime_str = "2025-06-18T10:30:00Z";
        let parsed_dt = parse_date_string(datetime_str).unwrap();
        assert_eq!(parsed_dt.format("%Y-%m-%dT%H:%M:%SZ").to_string(), "2025-06-18T10:30:00Z");
    }
}
