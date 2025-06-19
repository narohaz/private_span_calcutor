use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};

fn main() {
    let start_date: NaiveDate = Local::now().date_naive();
    let end_date: NaiveDate = NaiveDate::parse_from_str("2025-12-20", "%Y-%m-%d").unwrap();

    let mut date = start_date;

    let mut total_hour: i32 = 0;
    while date < end_date {
        let weekday = date.weekday();
        let hour = time_of_this_weekday(weekday, date);
        total_hour += i32::from(hour);
        date = date + Duration::days(1);
    }
    println!("{}", total_hour);
}

const WEEKDAY_HOUR: u8 = 4;
const WEEKEND_HOUR: u8 = 10;

fn time_of_this_weekday(day: Weekday, date: NaiveDate) -> u8 {
    match day {
        Weekday::Mon | Weekday::Tue | Weekday::Wed | Weekday::Thu | Weekday::Fri => WEEKDAY_HOUR,
        Weekday::Sat => {
            if is_this_saturday_work(date) {
                WEEKDAY_HOUR
            } else {
                WEEKEND_HOUR
            }
        }
        Weekday::Sun => WEEKEND_HOUR,
    }
}

/// 判断指定日期是否为工作日的星期六
fn is_this_saturday_work(date: NaiveDate) -> bool {
    let base_date = NaiveDate::parse_from_str("2025-06-21", "%Y-%m-%d").unwrap();
    let diff_days = (date - base_date).num_days().abs();
    let weeks = diff_days / 7;
    is_even(weeks)
}

/// 判断数字是否为偶数
fn is_even(n: i64) -> bool {
    n % 2 == 0
}
