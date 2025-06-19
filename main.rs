use chrono;
use chrono::{NaiveDate, Datelike, Weekday};

fn main() {
    // let today_date = chrono::Local::now().date_naive();
    // let today_weekday = today_date.weekday();

    // let start_date = chrono::Local::now().date_naive().and_hms(0, 0, 0);
    // let end_date = chrono::Local::now().date_naive().and_hms(23, 59, 59);

    // println!("{:?}", today_date);
    // println!("{:?}", today_weekday);
    // 传入一个日期字符串转成NaiveDate，然后获取对应的weekday
    let a_work_saturday_date = NaiveDate::parse_from_str("2025-06-21", "%Y-%m-%d").unwrap();
    let a_work_saturday_weekday = a_work_saturday_date.weekday();
    println!("{:?}", a_work_saturday_date);
    println!("{:?}", is_this_saturday_work(a_work_saturday_weekday));
}

// const WEEKDAY_HOUR: u8 = 6;
// const WEEKEND_HOUR: u8 = 10;

// const MONDAY_HOUR: u8 = WEEKDAY_HOUR;
// const TUESDAY_HOUR: u8 = WEEKDAY_HOUR;
// const WEDNESDAY_HOUR: u8 = WEEKDAY_HOUR;
// const THURSDAY_HOUR: u8 = WEEKDAY_HOUR;
// const FRIDAY_HOUR: u8 = WEEKDAY_HOUR;
// const SATURDAY_HOUR: u8 = if current_week_is_two_day_break() {WEEKEND_HOUR} else {WEEKDAY_HOUR};
// const SUNDAY_HOUR: u8 = WEEKEND_HOUR;

// fn time_of_this_weekday(day: chrono::Weekday, index: u8) -> u8 {
//     match day {
//         chrono::Weekday::Mon => WEEKDAY_HOUR,
//         chrono::Weekday::Tue => WEEKDAY_HOUR,
//         chrono::Weekday::Wed => WEEKDAY_HOUR,
//         chrono::Weekday::Thu => WEEKDAY_HOUR,
//         chrono::Weekday::Fri => WEEKDAY_HOUR,
//         chrono::Weekday::Sat => if last_week_is_one_day_break() {WEEKEND_HOUR} else {WEEKDAY_HOUR},
//         chrono::Weekday::Sun => WEEKEND_HOUR,
//     }
// }

// fn current_week_is_two_day_break(week: chrono::Weekday) -> bool {
    // if week == chrono::Weekday::Sat || week == chrono::Weekday::Sun 
// }

// 2025-06-21 是星期六 并且这天要工作
// 传入一个chrono::Weekday::Sat
fn is_this_saturday_work(&date: chrono::Weekday) -> bool {
    if date != chrono::Weekday::Sat {
        !panic!("date is not saturday");
    }

    let a_work_saturday_date = chrono::Local::("2025-06-21").unwrap();
    let diff = staturday - a_work_saturday_date.weekday();
    if is_even(diff.num_days() / 14) {
        return true;
    }
    return false;   
}
