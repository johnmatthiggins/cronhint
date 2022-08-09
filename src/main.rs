use std::vec::Vec;

struct CronExp {
    minute: ExpSeg,
    hour: ExpSeg,
    day: ExpSeg,
    month: ExpSeg,
    weekday: ExpSeg,
}

// Segment of CRON expression.
enum ExpSeg {
    List(Vec<usize>),
    Range(String, String),
    Frac(usize),
    Symbol(CronSymbol),
}

enum CronSymbol {
    Wildcard,
    Number(usize),
}

fn main() {
}

fn parse_list(exp: &String) -> Option<ExpSeg> {
    let mut fail = false;
    let chars: Vec<char> = exp.chars().collect();
    let mut output: Vec<usize> = Vec::new();
    let mut buffer: Vec<char> = Vec::new();

    let limit: usize = chars.len();
    let mut i = 0;

    while i < limit && !fail {
        let c = chars.get(i).unwrap();

        if c == &',' {
            if buffer.is_empty() {
                fail = true;
            }
            else {
                let num_str: String = buffer
                    .clone()
                    .into_iter()
                    .collect();
                buffer.clear();

                let num = num_str.parse::<usize>();

                if let Ok(n) = num {
                    output.push(n);
                }
                else {
                    fail = true;
                }
            }
        }
        else if c.is_digit(10) {
            buffer.push(*c);
        }
        else {
            fail = true;
        }
    }

    if fail {
        None
    }
    else {
        Some(ExpSeg::List(output))
    }
}

fn parse_range(exp: &String) -> Option<ExpSeg> {
    None
}

fn parse_frac(exp: &String) -> Option<ExpSeg> {
    let mut split_exp: Vec<&str> = exp.split("/").collect();

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
