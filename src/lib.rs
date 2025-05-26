use std::thread;
use std::time::Duration;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use chrono::{Datelike, NaiveDate,};

/// Listens for clipboard changes.
///
/// ```ignore
/// use format_date_for_irs::check_clipboard;
/// // gx todo: mock clipboard for reliable testing
/// check_clipboard(); // this runs indefinitely
/// ```
///
/// On change it will check if the clipboard contains a date.
/// Monitors the clipboard for date strings and reformats them for IRS compliance.
///
/// Continuously checks the clipboard for changes. If a new clipboard entry contains a date in `mm/dd/yy` format,
/// it parses and reformats the date to `mm/dd/YYYY` and updates the clipboard with the formatted value.
///
/// # Examples
///
/// ```ignore
/// // This function runs indefinitely and should be called from your main function.
/// check_clipboard();
/// ```
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

/// Parses a string into a date.
///
/// Returns `None` if the string does not match `mm/dd/yy`.
///
/// # Examples
///
/// ```
/// use format_date_for_irs::parse_date;
/// use chrono::NaiveDate;
///
/// let date = parse_date("01/02/21").unwrap();
/// assert_eq!(date, NaiveDate::from_ymd_opt(2021, 1, 2).unwrap());
/// ```
///
/// Attempts to parse a string as a date in `mm/dd/yy` format.
///
/// Trims leading and trailing whitespace from the input and tries to interpret it as a date using the `mm/dd/yy` format. Returns a `NaiveDate` if parsing is successful; otherwise, returns `None`.
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// let date = parse_date(" 12/31/23 ");
/// assert_eq!(date, Some(NaiveDate::from_ymd_opt(2023, 12, 31).unwrap()));
/// ```
pub fn parse_date(date: &str) -> Option<NaiveDate> {
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

/// Formats a date for the IRS.
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// use format_date_for_irs::format_date_for_irs;
///
/// let date = NaiveDate::from_ymd_opt(2021, 1, 2).unwrap();
/// assert_eq!(format_date_for_irs(&date), "01/02/2021");
/// ```
///
/// Formats a `NaiveDate` as a string in `mm/dd/YYYY` format for IRS compliance.
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// let date = NaiveDate::from_ymd_opt(2024, 5, 15).unwrap();
/// let formatted = format_date_for_irs(&date);
/// assert_eq!(formatted, "05/15/2024");
/// ```
pub fn format_date_for_irs(date: &NaiveDate) -> String {
    let date = NaiveDate::from_ymd_opt(date.year(), date.month(), date.day()).unwrap();
    let date = date.format("%m/%d/%Y").to_string();

    println!("Formatted date: {date}");

    date
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn parse_date_valid() {
        let expected = NaiveDate::from_ymd_opt(2021, 1, 2).unwrap();
        assert_eq!(parse_date("01/02/21"), Some(expected));
    }

    #[test]
    fn parse_date_failure() {
        assert!(parse_date("notadate").is_none());
        assert!(parse_date("13/32/22").is_none());
    }

    #[test]
    /// Tests that `parse_date` correctly handles leading and trailing whitespace in the input string by ensuring trimmed and untrimmed inputs yield the same result.
    fn parse_date_trim() {
        let trimmed = parse_date("01/02/21");
        let spaced = parse_date(" 01/02/21 ");
        assert_eq!(trimmed, spaced);
    }

    #[test]
    /// Tests that `format_date_for_irs` correctly formats a `NaiveDate` as `mm/dd/YYYY`.
    ///
    /// # Examples
    ///
    /// ```
    /// let date = NaiveDate::from_ymd_opt(2021, 1, 2).unwrap();
    /// assert_eq!(format_date_for_irs(&date), "01/02/2021");
    /// ```
    fn format_date() {
        let date = NaiveDate::from_ymd_opt(2021, 1, 2).unwrap();
        assert_eq!(format_date_for_irs(&date), "01/02/2021");
    }
}
