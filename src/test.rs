use crate::parse_cron_exp;

#[test]
fn random_1() {
    let cron_str = String::from("0 4 8-14 * *");

    let result = parse_cron_exp(&cron_str)
        .unwrap()
        .to_string();
    
    assert_eq!(result, String::from("At 4:00 AM on every day-of-month from 8 through 14."));
}

#[test]
fn random_2() {
    let cron_str = String::from("0 22 * * 1-5");

    let result = parse_cron_exp(&cron_str)
        .unwrap()
        .to_string();

    assert_eq!(result, String::from("At 10:00 PM on every day-of-week from Monday through Friday."));
}

#[test]
fn random_3() {
    let cron_str = String::from("0 4 8-14 * *");

    let result = parse_cron_exp(&cron_str)
        .unwrap()
        .to_string();

    assert_eq!(result, String::from("At 4:00 AM on every day-of-month from 8 through 14."));
}

#[test]
fn random_4() {
    let cron_str = String::from("0 22 * * 1-5");

    let result = parse_cron_exp(&cron_str)
        .unwrap()
        .to_string();

    assert_eq!(result, String::from("At 10:00 PM on every day-of-week from Monday through Friday."));
}

#[test]
fn random_5() {
    let cron_str = String::from("0 0,12 1 */2 *");

    let result = parse_cron_exp(&cron_str)
        .unwrap()
        .to_string();

    assert_eq!(result, String::from("At minute 0 past hour 0 and 12 on day-of-month 1 in every 2nd month."));
}

#[test]
fn random_6() {
    let cron_str = String::from("0 0,12 1 */2 *");

    let result = parse_cron_exp(&cron_str)
        .unwrap()
        .to_string();

    assert_eq!(result, String::from("At minute 0 past hour 0 and 12 on day-of-month 1 in every 2nd month."));
}

#[test]
fn random_7() {
    let cron_str = String::from("15 14 1 * *");

    let result = parse_cron_exp(&cron_str)
        .unwrap()
        .to_string();

    assert_eq!(result, String::from("At 2:15 PM on day-of-month 1."));
}

#[test]
fn all_symbols() {
    let cron_str = String::from("5 4 1,2,3 1 2");

    let result = parse_cron_exp(&cron_str)
        .unwrap()
        .to_string();
    
    assert_eq!(result, String::from("At 4:05 AM on day-of-month 1, 2, and 3 and on Tuesday in January."));
}

#[test]
fn no_minute_day_or_weekday() {
    let cron_str = String::from("* 0 * 8 *");

    let result = parse_cron_exp(&cron_str)
        .unwrap()
        .to_string();
    
    assert_eq!(result, String::from("At every minute past 12:00 AM in August."));
}

#[test]
fn no_minute_or_weekday() {
    let cron_str = String::from("* 0 */2 8 *");

    let result = parse_cron_exp(&cron_str)
        .unwrap()
        .to_string();
    
    assert_eq!(
        result,
        String::from("At every minute past 12:00 AM on every 2nd day-of-month in August."));
}

#[test]
fn only_weekday() {
    let cron_str = String::from("* * * * */3");

    let result = parse_cron_exp(&cron_str)
        .unwrap()
        .to_string();
    
    assert_eq!(
        result,
        String::from("At every minute on every 3rd day-of-week."));
}

#[test]
fn only_day() {
    let cron_str = String::from("* * 1-5 * *");

    let result = parse_cron_exp(&cron_str)
        .unwrap()
        .to_string();
    
    assert_eq!(
        result,
        String::from("At every minute on every day-of-month from 1 through 5."));
}

#[test]
fn only_month() {
    let cron_str = String::from("* * * 11 *");

    let result = parse_cron_exp(&cron_str)
        .unwrap()
        .to_string();
    
    assert_eq!(
        result,
        String::from("At every minute in November."));
}

#[test]
fn only_minute() {
    let cron_str = String::from("*/4 * * * *");

    let result = parse_cron_exp(&cron_str)
        .unwrap()
        .to_string();
    
    assert_eq!(
        result,
        String::from("At every 4th minute."));
}
