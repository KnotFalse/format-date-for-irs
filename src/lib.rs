use std::thread;
use std::time::Duration;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use chrono::{Datelike, NaiveDate,};

/// Listens for clipboard changes
/// On change it will check if the clipboard contains a date
/// If it does, it will format the date for the IRS and copy it to the clipboard
pub fn check_clipboard()
{
    let mut clipboard = ClipboardContext::new().unwrap();
    let mut previous = clipboard.get_contents().unwrap();
    loop {
        let current = clipboard.get_contents().unwrap();
        if current != previous {
            if let Some(date) = parse_date(&current) {
                let formatted = format_date_for_irs(&date);
                clipboard.set_contents(formatted).unwrap();
            }
            previous = current;
        }
        thread::sleep( Duration::from_millis(100) );
    }
}

/// Parses a string into a date
/// Returns None if the string is not a date, otherwise returns Some(date)
fn parse_date(date: &str) -> Option<NaiveDate> {
    let date = date.trim();

    println!( "Scanned for date; found: {date:?}" );

    // Try to parse the date
    let date = NaiveDate::parse_from_str(date, "%m/%d/%y");
    if let Ok(date) = date {
        // print is a date (mm/dd/yy)
        println!("Date: {}", date.format("%m/%d/%y"));

        return Some(date);
    }

    println!("Not a date: {}", date.unwrap_err());

    None
}

/// Formats a date for the IRS
/// Date must be in the format mm/dd/yy
/// Returns a string in the format mm/dd/yyyy
fn format_date_for_irs(date: &NaiveDate) -> String {
    let date = NaiveDate::from_ymd_opt(date.year(), date.month(), date.day()).unwrap();
    let date = date.format("%m/%d/%Y").to_string();

    println!("Formatted date: {date}");

    date
}