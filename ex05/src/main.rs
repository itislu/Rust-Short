// (January, 31), (February, 28/29), ...
//

fn main() {
    let mut day: u32 = 5;
    let mut days_in_month: u32;
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
    let current_year: u32 = 2024;
    let current_month: u32 = 10;

    for year in 1..2024 {
        for month in 1..=12 {
            days_in_month = num_days_in_month(year, month);
            while day <= days_in_month {
                if day == 13 {
                    println!("Friday, {} 13, {}", month_names[(month - 1) as usize], year);
                }
                day += 7;
            }
            day -= days_in_month;
        }
    }
    for month in 1..current_month {
        days_in_month = num_days_in_month(current_year, month);
        while day <= days_in_month {
            if day == 13 {
                println!("Friday, {} 13, {}", month_names[(month - 1) as usize], current_year);
            }
            day += 7;
        }
        day -= days_in_month;
    }
    days_in_month = num_days_in_month(current_year, current_month);
    while day <= days_in_month {
        if day == 13 {
            println!("Friday, {} 13, {}", month_names[(current_month - 1) as usize], current_year);
        }
        day += 7;
    }
}

fn num_days_in_month(year: u32, month: u32) -> u32 {
    match month {
        0 => 0, // don't like
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
        _ => 0, // don't like
    }
}

fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
