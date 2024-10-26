use std::process::exit;

use calendar::{Calendar, CalendarLength};
use structopt::StructOpt;
use chrono::{Datelike, Local, NaiveDateTime};
use regex::Regex;

pub mod calendar;

#[derive(StructOpt)]
#[structopt(name = "cal")]
/// Display a calendar
struct Args {
    /// Calendar date [[MONTH] YEAR] | [MONTH(literal)] | [YEAR]
    date: Vec<String>,
    /// Show a single month [DEFAULT]
    #[structopt(short = "1", long = "one")]
    show_month: bool,
    /// Show three months
    #[structopt(short = "3", long = "three")]
    show_quarter: bool,
    /// Show whole year
    #[structopt(short = "y", long = "year")]
    show_year: bool,
}

#[paw::main]
fn main(args: Args) {
    let cal = parse_date(&args.date);
    if cal.is_err() {
        eprintln!("{}", cal.unwrap_err());
        exit(1);
    }

    let cal = cal.unwrap().with_length(parse_length(&args));
    println!("{}", cal)
}

fn parse_length(args: &Args) -> CalendarLength {
    return if args.show_year { CalendarLength::YEAR }
        else if args.show_quarter { CalendarLength::QUARTER }
        else if args.show_month { CalendarLength::MONTH }
        else { CalendarLength::MONTH }
}

fn parse_date(date: &Vec<String>) -> Result<Calendar, String> {
    let now = Local::now();
    if date.len() == 0 {
        return Ok(Calendar::new(now.year(), now.month()))
    }

    if date.len() == 1 {
        if is_numeric(&date[0]) {
            return match parse_year(&date[0]) {
                Ok(year) => {
                    let cal = Calendar::new(year, now.month())
                        .with_length(CalendarLength::YEAR);
                    Ok(cal)
                },
                Err(msg) => Err(msg),
            }
        }
        return match parse_month_name(&date[0]) {
            Ok(month) => Ok(Calendar::new(now.year(), month)),
            Err(e) => Err(e),
        }
    }

    if !is_numeric(&date[1]) {
        return Err(format!("Invalid year: {}", &date[1]))
    }

    let month = if is_numeric(&date[0]) {
        parse_month(&date[0])
    } else {
        parse_month_name(&date[0])
    };
    if month.is_err() {
        return Err(month.unwrap_err())
    }
    
    match parse_year(&date[1]) {
        Ok(year) => Ok(Calendar::new(year, month.unwrap())),
        Err(msg) => Err(msg),
    }
}

fn parse_year(year: &str) -> Result<i32, String> {
    let year = year.trim();
    let max_year = NaiveDateTime::MAX.year();
    if year.len() > max_year.to_string().len() {
        return Err(format!("Exceeded max year: {}", year))
    }
    
    let year: i32 = year.parse().unwrap();
    if year > max_year {
        return Err(format!("Exceeded max year: {}", year))
    }

    Ok(year)
}

fn parse_month_name(month: &str) -> Result<u32, String> {
    let month = month.trim().to_lowercase();
    if month.len() < 3 || month.len() > 9 {
        return Err(format!("Unknown month name: {}", month));
    }

    let months = if month.len() == 3 {
        vec![
            "jan", "feb", "mar", "apr", "may", "jun",
            "jul", "aug", "sep", "oct", "nov", "dec"
        ]
    } else {
        vec![
            "january", "february", "march", "april", "may", "june",
            "july", "august", "september", "october", "november", "december"
        ]
    };
    let index = months.iter().position(|v| v == &month);
    
    if index.is_none() {
        return Err(format!("Unknown month name: {}", month))
    }
    let parsed_month: u32 = (index.unwrap() + 1).try_into().unwrap();

    Ok(parsed_month)
}

fn parse_month(month: &str) -> Result<u32, String> {
    let month = month.trim();
    let parsed_month: Result<u32, _> = month.parse();
    if parsed_month.is_err() {
        return Err(format!("Invalid month: {}", month))
    }

    let parsed_month = parsed_month.unwrap();
    if parsed_month < 1 || parsed_month > 12 {
        return Err(format!("Invalid month: {}", month))
    }

    Ok(parsed_month)
}

fn is_numeric(text: &str) -> bool {
    Regex::new(r"^\d+$").unwrap().is_match(text.trim())
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    use chrono::{Datelike, Local};

    #[test]
    fn test_parse_date() {
        let now = Local::now();

        // No date provided
        let expected = Ok(
            Calendar::new(now.year(), now.month()).with_length(CalendarLength::MONTH)
        );
        let params = vec![];
        let result = parse_date(&params);
        assert_eq!(expected, result);

        // Numeric month and year
        let expected = Ok(
            Calendar::new(2012, 5).with_length(CalendarLength::MONTH)
        );
        let params = vec![String::from("5"), String::from("2012")];
        let result = parse_date(&params);
        assert_eq!(expected, result);
        
        // Only year
        let expected = Ok(
            Calendar::new(2024, now.month()).with_length(CalendarLength::YEAR)
        );
        let params = vec![String::from("2024")];
        let result = parse_date(&params);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_date_literal_month() {
        let now = Local::now();

        // Long literal month and numeric year
        let expected = Ok(
            Calendar::new(2022, 2).with_length(CalendarLength::MONTH)
        );
        let params = vec![String::from("february"), String::from("2022")];
        let result = parse_date(&params);
        assert_eq!(expected, result);
        
        // Short literal month and numeric year
        let expected = Ok(
            Calendar::new(2022, 8).with_length(CalendarLength::MONTH)
        );
        let params = vec![String::from("aug"), String::from("2022")];
        let result = parse_date(&params);
        assert_eq!(expected, result);

        // Only long literal month
        let expected = Ok(
            Calendar::new(now.year(), 8).with_length(CalendarLength::MONTH)
        );
        let params = vec![String::from("august")];
        let result = parse_date(&params);
        assert_eq!(expected, result);

        // Only short literal month
        let expected = Ok(
            Calendar::new(now.year(), 8).with_length(CalendarLength::MONTH)
        );
        let params = vec![String::from("aug")];
        let result = parse_date(&params);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_date_invalid() {
        // Inverted month (numeric) and year
        let expected = Err(String::from("Invalid month: 2024"));
        let params = vec![String::from("2024"), String::from("08")];
        let result = parse_date(&params);
        assert_eq!(expected, result);

        // Invalid month (numeric) and valid year
        let expected = Err(String::from("Invalid month: 13"));
        let params = vec![String::from("13"), String::from("2024")];
        let result = parse_date(&params);
        assert_eq!(expected, result);

        // Invalid month (short literal) and valid year
        let expected = Err(String::from("Unknown month name: ocv"));
        let params = vec![String::from("ocv"), String::from("2024")];
        let result = parse_date(&params);
        assert_eq!(expected, result);

        // Invalid month (long literal) and valid year
        let expected = Err(String::from("Unknown month name: jabuary"));
        let params = vec![String::from("jabuary"), String::from("2024")];
        let result = parse_date(&params);
        assert_eq!(expected, result);

        // Invalid month (short literal)
        let expected = Err(String::from("Unknown month name: ocv"));
        let params = vec![String::from("ocv")];
        let result = parse_date(&params);
        assert_eq!(expected, result);

        // Invalid month (long literal)
        let expected = Err(String::from("Unknown month name: jabuary"));
        let params = vec![String::from("jabuary")];
        let result = parse_date(&params);
        assert_eq!(expected, result);

        // Valid month (numeric) and invalid year
        let expected = Err(String::from("Invalid year: abcd"));
        let params = vec![String::from("13"), String::from("abcd")];
        let result = parse_date(&params);
        assert_eq!(expected, result);

        // Exceeded year
        let expected = Err(String::from("Exceeded max year: 99999999"));
        let params = vec![String::from("99999999")];
        let result = parse_date(&params);
        assert_eq!(expected, result);
        
        // Valid month (numeric) and exceeded year
        let expected = Err(String::from("Exceeded max year: 99999999"));
        let params = vec![String::from("12"), String::from("99999999")];
        let result = parse_date(&params);
        assert_eq!(expected, result);

        // Valid month (short literal) and exceeded year
        let expected = Err(String::from("Exceeded max year: 99999999"));
        let params = vec![String::from("dec"), String::from("99999999")];
        let result = parse_date(&params);
        assert_eq!(expected, result);

        // Valid month (long literal) and exceeded year
        let expected = Err(String::from("Exceeded max year: 99999999"));
        let params = vec![String::from("december"), String::from("99999999")];
        let result = parse_date(&params);
        assert_eq!(expected, result);        
    }

    #[test]
    fn test_parse_year() {
        assert_eq!(Ok(2024), parse_year("2024"));
        assert_eq!(Ok(2024), parse_year("2024  "));
        assert_eq!(Ok(2024), parse_year("  2024"));
        assert_eq!(Ok(2024), parse_year("  2024  "));

        let err_msg = "Exceeded max year: ";
        assert_eq!(Err(format!("{}{}", err_msg, 999999)), parse_year("999999"));

        let invalid_year = NaiveDateTime::MAX.year() + 1;
        let expected = Err(format!("{}{}", err_msg, invalid_year));
        let result = parse_year(&invalid_year.to_string());
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_month_name() {
        assert_eq!(Ok(1), parse_month_name("january"));
        assert_eq!(Ok(2), parse_month_name("february"));
        assert_eq!(Ok(3), parse_month_name("march"));
        assert_eq!(Ok(4), parse_month_name("april"));
        assert_eq!(Ok(5), parse_month_name("may"));
        assert_eq!(Ok(6), parse_month_name("june",));
        assert_eq!(Ok(7), parse_month_name("july"));
        assert_eq!(Ok(8), parse_month_name("august"));
        assert_eq!(Ok(9), parse_month_name("september"));
        assert_eq!(Ok(10), parse_month_name("   october"));
        assert_eq!(Ok(11), parse_month_name("november   "));
        assert_eq!(Ok(12), parse_month_name("  december  "));

        assert_eq!(Ok(1), parse_month_name("jan"));
        assert_eq!(Ok(2), parse_month_name("feb"));
        assert_eq!(Ok(3), parse_month_name("mar"));
        assert_eq!(Ok(4), parse_month_name("apr"));
        assert_eq!(Ok(5), parse_month_name("may"));
        assert_eq!(Ok(6), parse_month_name("jun",));
        assert_eq!(Ok(7), parse_month_name("jul"));
        assert_eq!(Ok(8), parse_month_name("aug"));
        assert_eq!(Ok(9), parse_month_name("sep"));
        assert_eq!(Ok(10), parse_month_name("   oct"));
        assert_eq!(Ok(11), parse_month_name("nov   "));
        assert_eq!(Ok(12), parse_month_name("  dec  "));

        let err_msg = "Unknown month name: ";
        assert_eq!(Err(format!("{}{}", err_msg, "aaa")), parse_month_name("aaA"));
        assert_eq!(Err(format!("{}{}", err_msg, "januar")), parse_month_name("januar"));
    }

    #[test]
    fn test_parse_month() {
        assert_eq!(Ok(1), parse_month("1"));
        assert_eq!(Ok(2), parse_month("2"));
        assert_eq!(Ok(3), parse_month("3"));
        assert_eq!(Ok(4), parse_month("4"));
        assert_eq!(Ok(5), parse_month("5"));
        assert_eq!(Ok(6), parse_month("6"));
        assert_eq!(Ok(7), parse_month("0007"));
        assert_eq!(Ok(8), parse_month("008"));
        assert_eq!(Ok(9), parse_month("09"));
        assert_eq!(Ok(10), parse_month("10   "));
        assert_eq!(Ok(11), parse_month("  11"));
        assert_eq!(Ok(12), parse_month(" 12 "));

        let err_msg = "Invalid month: ";
        assert_eq!(Err(format!("{}{}", err_msg, 0)), parse_month("0"));
        assert_eq!(Err(format!("{}{}", err_msg, 13)), parse_month("13"));
        assert_eq!(Err(format!("{}{}", err_msg, 2024)), parse_month("  2024 "));
    }

    #[test]
    fn test_is_numeric() {
        assert!(is_numeric("78"));
        assert!(is_numeric("0"));
        assert!(is_numeric("  22  "));
        assert!(is_numeric("01"));
        
        assert!(!is_numeric(""));
        assert!(!is_numeric("text "));
    }

}