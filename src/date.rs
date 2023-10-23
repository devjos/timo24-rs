use chrono::{DateTime, Local};

pub fn today() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%d.%m.%Y").to_string()
}

#[cfg(test)]
mod tests {
    use crate::date::today;
    use regex::Regex;

    #[test]
    fn get_current_date() {
        let regex = Regex::new("[0-3][0-9]\\.[0-1][0-9]\\.[0-9]{4}").unwrap();
        assert!(regex.is_match(&today()));
    }
}
