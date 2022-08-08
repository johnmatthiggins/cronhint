use std::vec::Vec;

struct CronExp {
}

// Segment of CRON expression.
enum ExpSeg {
    List(Vec<usize>),
    Range(String, String),
    Frac(String, String),
    Symbol(String),
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
    
}

fn parse_frac(exp: &String) -> Option<ExpSeg> {
    None
}

fn parse_sym(exp: &String) -> Option<ExpSeg> {
}