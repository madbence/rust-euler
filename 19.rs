fn main() {
    let mut year = 1900;
    let mut day = 0;
    let mut dotw = 0;
    let mut month = 0;
    let mut sundays = 0;
    while year < 2001 {
        if dotw == 6 && day == 0 && year > 1900 {
            sundays += 1;
        }
        println!("{}.{}.{} ({})", year, month + 1, day + 1, dotw);
        day += 1;
        dotw = (dotw + 1) % 7;
        if days(year, month) <= day {
            day = 0;
            month += 1;
        }
        if month > 11 {
            month = 0;
            year += 1;
        }
    }
    println!("{}", sundays);
}

fn days(year: u32, month: u32) -> u32 {
    match month {
        1 => if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 29 } else { 28 },
        3|5|8|10 => 30,
        _ => 31
    }
}
