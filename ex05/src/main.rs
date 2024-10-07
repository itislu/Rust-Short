fn main() {
    let current_year: u32 = 2024;
    let current_month: u32 = 10;
    let current_day: u32 = 7;
    let month_names = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
        ];
    let mut day: u32 = 5;

    for year in 1..=2024 {
        let end_month = if year == current_year { current_month } else { 12 };
        for month in 1..=end_month {
            let end_day = if year == current_year && month == current_month {
                current_day
            } else {
                num_days_in_month(year, month)
            };
            while day <= end_day {
                if day == 13 {
                    println!("Friday, {} 13, {}", month_names[(month - 1) as usize], year);
                }
                day += 7;
            }
            day -= end_day;
        }
    }
}

fn num_days_in_month(year: u32, month: u32) -> u32 {
    match month {
        0 => 0, // don't like it
        1 => 31,
        2 if is_leap_year(year) => 29,
        2 => 28,
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => 0, // don't like it
    }
}

fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
