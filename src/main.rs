use std::process::exit;

use calendar::CalendarLength;
use structopt::StructOpt;

mod calendar;
mod parser;

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
    /// Disable colored output
    #[structopt(long)]
    no_color: bool
}

#[paw::main]
fn main(args: Args) {
    if args.no_color {
        colored::control::set_override(false);
    }

    let cal = parser::parse_date(&args.date);
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
