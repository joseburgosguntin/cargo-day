use std::fmt::Display;

use chrono::{Datelike, Local};
use clap::{Parser, ValueEnum};
use color_print::cprintln;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
enum DateOrder {
    DMY,
    MDY,
    YMD,
}

impl Display for DateOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = format!("{self:?}");
        debug.make_ascii_lowercase();
        write!(f, "{debug}")
    }
}

#[derive(Serialize, Deserialize)]
struct Config {
    order: DateOrder,
    separator: char,
    extension: String,
}

/// Touch a file with today's date
#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(version, about, long_about = None)]
struct Args {
    /// Order of day, month and year in date
    #[arg(short, long)]
    order: Option<DateOrder>,

    /// Character to sepate the day month and year by
    #[arg(short, long)]
    separator: Option<char>,

    /// Character to sepate the day month and year by
    #[arg(short, long)]
    extension: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            order: DateOrder::DMY,
            separator: '-',
            extension: String::from("md"),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let Config {
        order,
        separator,
        extension,
    } = confy::load("day", None)?;
    let args = Args::parse();
    let order = args.order.unwrap_or(order);
    let sep = args.separator.unwrap_or(separator);
    let ext = args.extension.unwrap_or(extension);

    let date = Local::now();
    let d = format!("{:02}", date.day());
    let m = format!("{:02}", date.month());
    let y = format!("{:04}", date.year());

    let file_name = match order {
        DateOrder::DMY => format!("{d}{sep}{m}{sep}{y}"),
        DateOrder::MDY => format!("{m}{sep}{d}{sep}{y}"),
        DateOrder::YMD => format!("{y}{sep}{m}{sep}{d}"),
    };
    cprintln!("<b>Successfully created <m>{}.{}</>", file_name, ext);

    std::fs::File::create(format!("{file_name}.{ext}"))?;

    Ok(())
}
