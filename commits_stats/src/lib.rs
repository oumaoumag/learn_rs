use std::collections::HashMap;
use chrono::{NaiveDateTime, Datelike, Weekday};

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut result = HashMap::new();

    for commit in data.members() {
        if let Some(login) = commit["author"]["login"].as_str() {
            *result.entry(login.to_string()).or_insert(0) += 1;
        }
    }

    result
}

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut result = HashMap::new();

    for commit in data.members() {
        if let Some(date_str) = commit["commit"]["author"]["date"].as_str() {
            if let Ok(date) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%SZ") {
                let year = date.year();
                let week = date.iso_week().week();

                let week_str = format!("{}-W{}", year, week);

                *result.entry(week_str).or_insert(0) += 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_commits_per_author() {
        let contents = fs::read_to_string("commits.json").unwrap();
        let serialized = json::parse(&contents).unwrap();
        let result = commits_per_author(&serialized);

        // Check that we have some authors
        assert!(!result.is_empty());

        // Check specific authors from our sample data
        assert_eq!(*result.get("octocat").unwrap_or(&0), 2);
        assert_eq!(*result.get("johndoe").unwrap_or(&0), 1);
    }

    #[test]
    fn test_commits_per_week() {
        let contents = fs::read_to_string("commits.json").unwrap();
        let serialized = json::parse(&contents).unwrap();
        let result = commits_per_week(&serialized);

        // Check that we have some weeks
        assert!(!result.is_empty());

        // Check specific weeks from our sample data
        assert!(result.contains_key("2020-W44") || result.contains_key("2020-W45") ||
                result.contains_key("2020-W46"));
    }
}