use time::{PrimitiveDateTime as DateTime, Duration, Date, Time};

fn main(){
    let some_day = dt(2024, 4, 7, 7, 36, 0);
    println!("{}", after(some_day));
}

fn dt(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> DateTime {
    DateTime::new(
        Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(), 
        Time::from_hms(hour, minute, second).unwrap()
    )
}

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + Duration::new(1_000_000_000, 0)
}
