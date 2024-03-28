use chrono::{DateTime, Utc};
use chrono_tz::Tz;

pub struct TimeConverter {
    time: DateTime<Utc>,
}

impl TimeConverter {
    pub fn new(time: DateTime<Utc>) -> TimeConverter {
        TimeConverter { time }
    }

    pub fn to_jst(&self) -> String {
        let now_jst = &self.time.with_timezone(&Tz::Japan);

        format!("{}", now_jst.format("%Y-%m-%d %H:%M:%S"))
    }

    pub fn to_us_central(&self) -> String {
        let now_us_central = &self.time.with_timezone(&Tz::US__Central);

        format!("{}", now_us_central.format("%Y-%m-%d %H:%M:%S"))
    }

    pub fn show(&self) -> String {
        format!("JST:{}, CST:{}", &self.to_jst(), &self.to_us_central())
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn test_time_converter() {
        let time: DateTime<Utc> = Utc.with_ymd_and_hms(2024, 03, 10, 9, 0, 0).unwrap();
        let result = TimeConverter::new(time);

        assert_eq!(result.to_jst(), "2024-03-10 18:00:00");
        assert_eq!(result.to_us_central(), "2024-03-10 04:00:00");
        assert_eq!(
            result.show(),
            "JST:2024-03-10 18:00:00, CST:2024-03-10 04:00:00"
        );
    }
}
