use chrono::Datelike;
use clap::Parser;
use inquire::InquireError;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    year: Option<usize>,
    day: Option<usize>,
}

#[quit::main]
fn main() {
    let mut args = Args::parse();
    let timestamp = chrono::offset::Utc::now() - chrono::Duration::hours(5);

    if args.year.is_none() {
        args.year = Some(timestamp.year().try_into().unwrap());
        args.day = Some(timestamp.day().try_into().unwrap());
    }

    if args.day.is_none() {
        args.day = Some(timestamp.day().try_into().unwrap());
    }

    // std::path::Path::new(format!("./{args"))

    loop {
        if args.year.is_some_and(|y| y >= 2015)
            && args.year.is_some_and(|y| {
                y <= match timestamp.month() {
                    12 => timestamp.year().try_into().unwrap(),
                    _ => (timestamp.year() - 1).try_into().unwrap(),
                }
            })
        {
            break;
        }

        args.year = inquire::Text::new(&format!(
            "Enter desired year (2015 - {}):",
            match timestamp.month() {
                12 => timestamp.year(),
                _ => timestamp.year() - 1,
            }
        ))
        .prompt()
        .map_err(|e| match e {
            InquireError::OperationCanceled | InquireError::OperationInterrupted => {
                eprintln!("Quitting!");
                quit::with_code(1);
            }
            _ => e,
        })
        .ok()
        .and_then(|s| s.parse::<usize>().ok());
    }

    loop {
        if args.day.is_some_and(|d| d >= 1) && args.day.is_some_and(|d| d <= 25) {
            break;
        }

        args.day = inquire::Text::new("Enter desired day (1 - 25):")
            .prompt()
            .map_err(|e| match e {
                InquireError::OperationCanceled | InquireError::OperationInterrupted => {
                    eprintln!("Quitting!");
                    quit::with_code(1);
                }
                _ => e,
            })
            .ok()
            .and_then(|s| s.parse::<usize>().ok());
    }

    println!("You picked {} - {}", args.year.unwrap(), args.day.unwrap());
}
