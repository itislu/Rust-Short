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
        let end_month = if year == current_year {
            current_month
        } else {
            12
        };
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

#[cfg(test)]
mod num_days_in_month {
    use super::*;

    #[test]
    fn test_leap_year_february() {
        let result = num_days_in_month(4, 2);
        assert_eq!(result, 29);
        let result = num_days_in_month(1600, 2);
        assert_eq!(result, 29);
    }

    #[test]
    fn test_common_year_february() {
        let result = num_days_in_month(5, 2);
        assert_eq!(result, 28);
        let result = num_days_in_month(1601, 2);
        assert_eq!(result, 28);
    }

    #[test]
    fn test_leap_year_other() {
        let result = num_days_in_month(4, 1);
        assert_eq!(result, 31);
        let result = num_days_in_month(1600, 12);
        assert_eq!(result, 31);
    }

    #[test]
    fn test_common_year_other() {
        let result = num_days_in_month(5, 1);
        assert_eq!(result, 31);
        let result = num_days_in_month(1601, 12);
        assert_eq!(result, 31);
    }

    #[test]
    #[should_panic]
    fn test_invalid_month() {
        let _result = num_days_in_month(5, 13);
        let _result = num_days_in_month(1600, 112);
    }
}

fn is_leap_year(year: u32) -> bool {
    if year == 0 {
        panic!()
    } else {
        (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    }
}

#[cfg(test)]
mod is_leap_year {
    use super::*;

    #[test]
    fn test_leap_year() {
        let result = is_leap_year(1600);
        assert_eq!(result, true);
        let result = is_leap_year(2004);
        assert_eq!(result, true);
    }

    #[test]
    fn test_common_year() {
        let result = is_leap_year(1500);
        assert_eq!(result, false);
        let result = is_leap_year(2003);
        assert_eq!(result, false);
    }

    #[test]
    fn test_leap_year_month() {
        let result = is_leap_year(1500);
        assert_eq!(result, false);
        let result = is_leap_year(2003);
        assert_eq!(result, false);
    }

    #[test]
    #[should_panic]
    fn test_year_zero() {
        let _result = is_leap_year(0);
    }
}
