mod test;

use std::vec::Vec;
use std::io::{self, BufRead};
use clap::Parser;

#[derive(Debug)]
struct CronExp {
    minute: ExpValue,
    hour: ExpValue,
    day: ExpValue,
    month: ExpValue,
    weekday: ExpValue,
}

// Segment of CRON expression.
#[derive(Debug, Clone)]
enum ExpValue {
    List(Vec<usize>),
    Range(usize, usize),
    Frac(usize),
    Symbol(CronSymbol),
}

#[derive(Debug, Clone)]
enum CronSymbol {
    Wildcard,
    Number(usize),
}

/// CLI tool for translating CRON expressions to plain english.
#[derive(Parser, Debug)]
#[clap(version, author, about, long_about = None)]
struct Args {
    /// Read in CRON expression from standard input.
    #[clap(short, long, action = clap::ArgAction::Count)]
    input: u8,
    
    /// CRON expression to translate.
    #[clap(value_parser, value_name = "EXPRESSION")]
    expression: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut cron_str;

    // If configured to accept input from STDIN.
    if args.input > 0 {
        cron_str = String::new();
        io::stdin()
            .lock()
            .read_line(&mut cron_str)
            .unwrap();

        cron_str = cron_str.trim().to_string();
    }
    else {
        cron_str = args.expression.unwrap();
    }

    let cron_exp = parse_cron_exp(&cron_str);

    if let Some(exp) = cron_exp {
        println!("{}", exp.to_string());
    }
    else {
        println!("Expression could not be parsed!");
    }
}

fn print_daytime(minute: &ExpValue, hour: &ExpValue) -> String {
    match (minute, hour) {
        (ExpValue::Symbol(cs_min), _) => print_symbol_first(&cs_min, &hour),
        (ExpValue::List(m_list), _) => format!("At minute {} {}",
                                                            join_oxford_comma(&m_list),
                                                            hour.hour_str()),
        (ExpValue::Range(m_start, m_end), _) => format!("At every minute from {} through {} {}",
                                m_start, m_end, hour.hour_str()),
        (ExpValue::Frac(m_frac), _) =>
            format!("At every {} minute{}",
                with_ordinal_postfix(&m_frac), left_pad_if_not_empty(hour.hour_str())),
    }
}

fn left_pad_if_not_empty(text: String) -> String {
    if text.is_empty() {
        text
    }
    else {
        format!(" {}", text)
    }
}

fn print_symbol_first(minute: &CronSymbol, hour: &ExpValue) -> String {
    match hour {
        ExpValue::Symbol(hour_sym) => print_daytime_symbols(minute, hour_sym),
        _ => match minute {
            CronSymbol::Wildcard => format!("At every minute {}", hour.hour_str()),
            CronSymbol::Number(m) => format!("At minute {} {}", m, hour.hour_str()),
        },
    }
}

fn join_oxford_comma<T: ToString>(list: &Vec<T>) -> String {
    if list.len() > 1 {
        let mut comma_sep: Vec<String> = Vec::new();

        for i in 0..(list.len()) {
            let num = list
                .get(i)
                .unwrap()
                .to_string();

            // If last.
            if i == list.len() - 1 {
                let segment = format!("and {}", num).to_string();
                comma_sep.push(segment);
            }
            else {
                if i == list.len() - 2 && list.len() == 2 {
                    let segment = format!("{}", num).to_string();
                    comma_sep.push(segment);
                }
                else {
                    let segment = format!("{},", num).to_string();
                    comma_sep.push(segment);
                }
            }
        }

        comma_sep.join(" ")
    }
    else if list.len() == 1 {
        list.first().unwrap().to_string()
    }
    else {
        // Idk this shouldn't happen.
        panic!("ERROR");
    }
}

fn weekday_name(day: &usize) -> String {
    match day % 7 {
        1 => String::from("Monday"),
        2 => String::from("Tuesday"),
        3 => String::from("Wednesday"),
        4 => String::from("Thursday"),
        5 => String::from("Friday"),
        6 => String::from("Saturday"),
        _ => String::from("Sunday")
    }
}

fn month_name(month: &usize) -> String {
    match month % 12 {
        1 => String::from("January"),
        2 => String::from("February"),
        3 => String::from("March"),
        4 => String::from("April"),
        5 => String::from("May"),
        6 => String::from("June"),
        7 => String::from("July"),
        8 => String::from("August"),
        9 => String::from("September"),
        10 => String::from("October"),
        11 => String::from("November"),
        _ => String::from("December"),
    }
}

fn print_daytime_symbols(minute: &CronSymbol, hour: &CronSymbol) -> String {
    match (minute, hour) {
        (CronSymbol::Wildcard, CronSymbol::Wildcard) => String::from("At every minute"),
        (CronSymbol::Wildcard, CronSymbol::Number(h_n)) =>
            format!("At every minute past {}", time_with_am_pm(h_n, &0)),
        (CronSymbol::Number(m_n), CronSymbol::Number(h_n)) =>
            format!("At {}", time_with_am_pm(m_n, h_n)),
        (CronSymbol::Number(m_n), CronSymbol::Wildcard) => format!("At minute {} every hour", m_n),
    }
}

fn time_with_am_pm(minutes: &usize, hour: &usize) -> String {
    match hour % 24 {
        0 => format!("12:{:0>#2} AM", minutes),
        12 => format!("{}:{:0>#2} PM", hour, minutes),
        13 ..= 23 => format!("{}:{:0>#2} PM", (hour % 24) - 12, minutes),
        _ => format!("{}:{:0>#2} AM", hour, minutes),
    }
}

fn with_ordinal_postfix(number: &usize) -> String {
    match number {
        11 => String::from("11th"),
        12 => String::from("12th"),
        13 => String::from("13th"),
        _ => {
            let last_character: char = number
                .to_string()
                .chars()
                .last()
                .unwrap();

            match last_character {
                '0' | '4' | '5' | '6' | '7' | '8' | '9'
                    => format!("{}th", number.to_string()),
                '1' => format!("{}st", number.to_string()),
                '2' => format!("{}nd", number.to_string()),
                '3' => format!("{}rd", number.to_string()),
                _ => panic!("CANNOT PARSE!!!"),
            }
        },
    }
}

trait CronDisplay {
    fn hour_str(&self) -> String;
    fn weekday_str(&self) -> String;
    fn day_str(&self) -> String;
    fn month_str(&self) -> String;
}

impl CronDisplay for ExpValue {
    fn hour_str(&self) -> String {
        match self {
            ExpValue::List(list) =>
                format!("past hour {}", join_oxford_comma(list)),
            ExpValue::Range(start, end) =>
                format!("past every hour from {} through {}", start, end),
            ExpValue::Frac(div) =>
                format!("past every {} hour", with_ordinal_postfix(div)),
            _ => String::from(""),
        }
    }

    fn weekday_str(&self) -> String {
        match self {
            ExpValue::List(list) => format!("on {}", 
                                         join_oxford_comma(&list
                                                           .iter()
                                                           .map(|x| weekday_name(x))
                                                           .collect())),
            ExpValue::Range(start, end) => format!("on every day-of-week from {} through {}",
                                                                weekday_name(&start), weekday_name(&end)),
            ExpValue::Frac(div) => format!("on every {} day-of-week",
                                                        with_ordinal_postfix(&div)),
            ExpValue::Symbol(weekday) => match weekday {
                CronSymbol::Wildcard => String::from(""),
                CronSymbol::Number(n) => format!("on {}", weekday_name(&n)),
            }
        }
    }

    fn day_str(&self) -> String {
        match self {
            ExpValue::List(list) =>
                format!("on day-of-month {}", join_oxford_comma(list)),
            ExpValue::Frac(div) =>
                format!("on every {} day-of-month", with_ordinal_postfix(&div)),
            ExpValue::Range(start, end) =>
                format!("on every day-of-month from {} through {}", start, end),
            ExpValue::Symbol(sym) => match sym {
                CronSymbol::Wildcard => String::from(""),
                CronSymbol::Number(n) => format!("on day-of-month {}", n),
            },
        }
    }

    fn month_str(&self) -> String {
        match self {
            ExpValue::List(list) => format!("in {}", join_oxford_comma(&list.iter()
                                                         .map(|x| month_name(x))
                                                         .collect())),
            ExpValue::Frac(div) => format!("in every {} month",
                                                with_ordinal_postfix(&div)),
            ExpValue::Range(start, end) =>
                format!("in every month from {} through {}", month_name(&start), month_name(&end)),
            ExpValue::Symbol(sym) => match sym {
                CronSymbol::Wildcard => String::from(""),
                CronSymbol::Number(n) => format!("in {}", month_name(&n)),
            },
        }
    }
}

impl ToString for CronExp {
    fn to_string(&self) -> String {
        let daytime = print_daytime(&self.minute, &self.hour);
        let day = self.day.day_str();
        let weekday = self.weekday.weekday_str();
        let month = self.month.month_str();

        let mut output = daytime.clone();
        
        if day.is_empty() {
            if weekday.is_empty() {
                if !month.is_empty() {
                    output += format!(" {}", month).as_str();
                }
            }
            else {
                output += format!(" {}", weekday).as_str();

                if !month.is_empty() {
                    output += format!(" {}", month).as_str();
                }
            }
        }
        else {
            output += format!(" {}", day).as_str();
            
            if weekday.is_empty() {
                if !month.is_empty() {
                    output += format!(" {}", month).as_str();
                }
            }
            else {
                output += format!(" and {}", weekday).as_str();
                if !month.is_empty() {
                    output += format!(" {}", month).as_str();
                }
            }
        }

        output + "."
    }
}

impl ToString for ExpValue {
    fn to_string(&self) -> String {
        match self {
            ExpValue::List(l) => l.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","),
            ExpValue::Range(a, b) => String::from(format!("{}-{}", a, b)),
            ExpValue::Frac(v) => String::from(format!("*/{}", v)),
            ExpValue::Symbol(s) => match s {
                CronSymbol::Wildcard => String::from("*"),
                CronSymbol::Number(n) => n.to_string(),
            },
        }
    }
}

fn parse_cron_exp(cron_exp: &String) -> Option<CronExp> {
    let cron_vec: Vec<&str> = cron_exp
        .split(" ")
        .collect();

    if cron_vec.len() == 5 {
        let cron_segs: Vec<ExpValue> = cron_vec
            .into_iter()
            .map(|x| parse_exp_seg(&x.to_string()))
            .filter(|x| matches!(x, Some(_)))
            .map(|x| x.unwrap())
            .collect();

        if cron_segs.len() == 5 {
            let result = CronExp {
                minute: cron_segs.get(0).unwrap().clone(),
                hour: cron_segs.get(1).unwrap().clone(),
                day: cron_segs.get(2).unwrap().clone(),
                month: cron_segs.get(3).unwrap().clone(),
                weekday: cron_segs.get(4).unwrap().clone(),
            };

            Some(result)
        }
        else {
            None
        }
    }
    else {
        None
    }
}

fn parse_exp_seg(exp: &String) -> Option<ExpValue> {
    if let Some(v) = parse_list(exp) {
        Some(v)
    }
    else if let Some(v) = parse_range(exp) {
        Some(v)
    }
    else if let Some(v) = parse_sym(exp) {
        Some(v)
    }
    else if let Some(v) = parse_frac(exp) {
        Some(v)
    }
    else {
        None
    }
}

fn parse_list(exp: &String) -> Option<ExpValue> {
    if exp.contains(",") {
        let list: Vec<&str> = exp
            .split(",")
            .collect();

        let result: Vec<usize> = list
            .clone()
            .into_iter()
            .map(|x| x.parse::<usize>().ok())
            .filter(|x| matches!(x, Some(_)))
            .map(|x| x.unwrap())
            .collect();

        if result.len() != list.len() {
            None
        }
        else {
            Some(ExpValue::List(result))
        }
    }
    else {
        None
    }
}

fn parse_range(exp: &String) -> Option<ExpValue> {
    let split_exp: Vec<&str> = exp.split("-").collect();

    let first: Option<usize> = split_exp
        .get(0)
        .and_then(|x| x.parse::<usize>().ok());
    let second: Option<usize> = split_exp
        .get(1)
        .and_then(|x| x.parse::<usize>().ok());

    // Combine into range expression.
    first.and_then(|a| second
            .map(|b| ExpValue::Range(a, b)))
}

fn parse_frac(exp: &String) -> Option<ExpValue> {
    let split_exp: Vec<&str> = exp.split("/").collect();

    if split_exp.first().map_or(false, |x| *x == "*") {
        split_exp
            .get(1)
            .and_then(|x| x.parse::<usize>().ok())
            .map(|x| ExpValue::Frac(x))
    }
    else {
        None
    }
}

fn parse_sym(exp: &String) -> Option<ExpValue> {
    if exp.as_str() == "*" {
        Some(ExpValue::Symbol(CronSymbol::Wildcard))
    }
    else {
        exp.parse::<usize>()
            .ok()
            .map(|n| ExpValue::Symbol(CronSymbol::Number(n)))
    }
}
