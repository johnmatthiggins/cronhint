use std::vec::Vec;
use std::env;

#[derive(Debug)]
enum TimeType {
    Minute,
    Hour,
    Day,
    Month,
    Weekday,
}

#[derive(Debug)]
struct CronExp {
    minute: ExpComponent,
    hour: ExpComponent,
    day: ExpComponent,
    month: ExpComponent,
    weekday: ExpComponent,
}

// Segment of CRON expression.
#[derive(Debug, Clone)]
enum ExpValue {
    List(Vec<usize>),
    Range(usize, usize),
    Frac(usize),
    Symbol(CronSymbol),
}

#[derive(Debug)]
struct ExpComponent {
    value: ExpValue,
    units: TimeType,
}

#[derive(Debug, Clone)]
enum CronSymbol {
    Wildcard,
    Number(usize),
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("USAGE: cronhint '* * * * *'");
    }
    else {
        let cron_str = args
            .get(1)
            .unwrap()
            .to_string();

        let cron_exp = parse_cron_exp(&cron_str);

        if let Some(exp) = cron_exp {
            println!("{}", exp.to_string());
        }
        else {
            println!("Expression could not be parsed!");
        }
    }
}

fn print_daytime(minute: &ExpValue, hour: &ExpValue) -> String {
    match (minute, hour) {
        (ExpValue::Symbol(cs_min), _) => print_symbol_first(&cs_min, &hour),
        (ExpValue::List(m_list), _) => String::from(format!("At minute {} {}",
                                                            join_oxford_comma(&m_list),
                                                            hour.hour_str())),
        (ExpValue::Range(m_start, m_end), _) =>
            String::from(format!("At every minute from {} through {} {}",
                                m_start, m_end, hour.hour_str())),
        (ExpValue::Frac(m_frac), _) =>
            String::from(format!("At every {} minute {}",
                                 with_ordinal_postfix(&m_frac), hour.hour_str())),
    }
}

fn print_symbol_first(minute: &CronSymbol, hour: &ExpValue) -> String {
    match hour {
        ExpValue::Symbol(hour_sym) => print_daytime_symbols(minute, hour_sym),
        _ => match minute {
            CronSymbol::Wildcard => String::from(format!("Every minute past {}", hour.hour_str())),
            CronSymbol::Number(m) => String::from(format!("At minute {} past {}", m, hour.hour_str())),
        },
    }
}

fn join_oxford_comma(list: &Vec<usize>) -> String {
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
                let segment = format!("{},", num).to_string();
                comma_sep.push(segment);
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

fn weekday_name(day: usize) -> String {
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

fn print_daytime_symbols(minute: &CronSymbol, hour: &CronSymbol) -> String {
    match (minute, hour) {
        (CronSymbol::Wildcard, CronSymbol::Wildcard) => String::from("At every minute"),
        (CronSymbol::Wildcard, CronSymbol::Number(h_n)) =>
            String::from(format!("At every minute {}", time_with_am_pm(h_n, &0))),
        (CronSymbol::Number(m_n), CronSymbol::Number(h_n)) => String::from(format!("At {}", time_with_am_pm(m_n, h_n))),
        (CronSymbol::Number(m_n), CronSymbol::Wildcard) => String::from(format!("At minute {} every hour", m_n)),
    }
}

fn print_daytime_symbol_fraction(minute: &CronSymbol, hour: &usize) -> String {
    match minute {
        CronSymbol::Wildcard =>
            String::from(format!(
                    "At every minute past every {} hour",
                    with_ordinal_postfix(hour))),
        CronSymbol::Number(n) =>
            String::from(format!(
                "At minute {} past every {} hour",
                n,
                with_ordinal_postfix(hour))),
    }
}

fn print_daytime_fraction_symbol(minute: &usize, hour: &CronSymbol) -> String {
    match hour {
        CronSymbol::Wildcard =>
            String::from(format!("At every {} minute",
                    with_ordinal_postfix(minute))),
        CronSymbol::Number(n) =>
            String::from(format!("At every {} minute past {}",
                    with_ordinal_postfix(minute), time_with_am_pm(n, &0))),
    }
}

fn time_with_am_pm(hour: &usize, minutes: &usize) -> String {
    match hour % 24 {
        0 => String::from(format!("12:{:0>#9} AM", minutes)),
        12 => String::from(format!("{}:{:0>#9} PM", hour, minutes)),
        13 ..= 23 => String::from(format!("{}:{:0>#9} PM", (hour % 24) - 11, minutes)),
        _ => String::from(format!("{}:{:0>#9} AM", hour, minutes)),
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
                    => String::from(format!("{}th", number.to_string())),
                '1' => String::from(format!("{}st", number.to_string())),
                '2' => String::from(format!("{}nd", number.to_string())),
                '3' => String::from(format!("{}rd", number.to_string())),
                _ => panic!("CANNOT PARSE!!!"),
            }
        },
    }
}

trait CronDisplay {
    fn hour_str(&self) -> String;
}

impl CronDisplay for ExpValue {
    fn hour_str(&self) -> String {
        match self {
            ExpValue::List(list) =>
                String::from(format!("past hour {}", join_oxford_comma(list))),
            ExpValue::Range(start, end) =>
                String::from(format!("past every hour from {} through {}", start, end)),
            ExpValue::Frac(div) =>
                String::from(format!("past every {} hour", with_ordinal_postfix(div))),
            _ => String::from(""),
        }
    }
}

impl ToString for CronExp {
    fn to_string(&self) -> String {
        // String::from(format!(
        //              "{} {} {} {} {}",
        //              self.minute.value.to_string(),
        //              self.hour.value.to_string(),
        //              self.day.value.to_string(),
        //              self.month.value.to_string(),
        //              self.weekday.value.to_string()))
        String::from(format!("{}", print_daytime(&self.minute.value, &self.hour.value)))
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
                minute: ExpComponent {
                    value: cron_segs.get(0).unwrap().clone(),
                    units: TimeType::Minute,
                },
                hour: ExpComponent {
                    value: cron_segs.get(1).unwrap().clone(),
                    units: TimeType::Hour,
                },
                day: ExpComponent {
                    value: cron_segs.get(2).unwrap().clone(),
                    units: TimeType::Day,
                },
                month: ExpComponent {
                    value: cron_segs.get(3).unwrap().clone(),
                    units: TimeType::Month,
                },
                weekday: ExpComponent { 
                    value: cron_segs.get(4).unwrap().clone(),
                    units: TimeType::Weekday,
                },
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
