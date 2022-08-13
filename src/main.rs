use std::vec::Vec;
use std::env;

#[derive(Debug)]
struct CronExp {
    minute: ExpSeg,
    hour: ExpSeg,
    day: ExpSeg,
    month: ExpSeg,
    weekday: ExpSeg,
}

// Segment of CRON expression.
#[derive(Debug, Clone)]
enum ExpSeg {
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

fn with_ordinal_postfix(number: &usize) -> String {
    match number {
        11 => String::from("11th"),
        12 => String::from("12th"),
        13 => String::from("13th"),
        _ => {
            let last_character: char = number
                .to_string()
                .chars()
                .collect()
                .last()
                .unwrap();

            match last_character {
                '0' | '4' | '5' | '6' | '7' | '8' | '9'
                    => String::from(format!("{}th", number.to_string())),
                '1' => String::from(format!("{}st", number.to_string())),
                '2' => String::from(format!("{}nd", number.to_string())),
                '3' => String::from(format!("{}rd", number.to_string())),
            }
        },
    }
}

impl ToString for CronExp {
    fn to_string(&self) -> String {
        String::from(format!(
                     "{} {} {} {} {}",
                     self.minute.to_string(),
                     self.hour.to_string(),
                     self.day.to_string(),
                     self.month.to_string(),
                     self.weekday.to_string()))
    }
}

impl ToString for ExpSeg {
    fn to_string(&self) -> String {
        match self {
            ExpSeg::List(l) => l.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","),
            ExpSeg::Range(a, b) => String::from(format!("{}-{}", a, b)),
            ExpSeg::Frac(v) => String::from(format!("*/{}", v)),
            ExpSeg::Symbol(s) => match s {
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
        let cron_segs: Vec<ExpSeg> = cron_vec
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

fn parse_exp_seg(exp: &String) -> Option<ExpSeg> {
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

fn parse_list(exp: &String) -> Option<ExpSeg> {
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
        Some(ExpSeg::List(result))
    }
}

fn parse_range(exp: &String) -> Option<ExpSeg> {
    let split_exp: Vec<&str> = exp.split("-").collect();

    let first: Option<usize> = split_exp
        .get(0)
        .and_then(|x| x.parse::<usize>().ok());
    let second: Option<usize> = split_exp
        .get(1)
        .and_then(|x| x.parse::<usize>().ok());

    // Combine into range expression.
    first.and_then(|a| second
            .map(|b| ExpSeg::Range(a, b)))
}

fn parse_frac(exp: &String) -> Option<ExpSeg> {
    let split_exp: Vec<&str> = exp.split("/").collect();

    if split_exp.first().map_or(false, |x| *x == "*") {
        split_exp
            .get(1)
            .and_then(|x| x.parse::<usize>().ok())
            .map(|x| ExpSeg::Frac(x))
    }
    else {
        None
    }
}

fn parse_sym(exp: &String) -> Option<ExpSeg> {
    if exp.as_str() == "*" {
        Some(ExpSeg::Symbol(CronSymbol::Wildcard))
    }
    else {
        exp.parse::<usize>()
            .ok()
            .map(|n| ExpSeg::Symbol(CronSymbol::Number(n)))
    }
}
