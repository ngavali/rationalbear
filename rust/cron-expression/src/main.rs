use core::panic;
use std::{collections::HashMap, ops::Add};

use chrono::{DateTime, Datelike, Local, NaiveDate, TimeZone, Timelike, Utc};

#[derive(Clone, Copy, Debug)]
enum CronFieldName {
    Minute,
    Hour,
    DayOfMonth,
    Month,
    DayOfWeek,
}

#[derive(Debug)]
enum ErrorCode {
    InvalidStep,
    InvalidRange,
    InvalidValue,
}

struct CronExpression {}

impl CronExpression {
    fn new() -> CronExpression {
        CronExpression {}
    }

    fn ranges() {
        let mut CRON_FIELD_RANGE: HashMap<CronFieldName, [u8; 2]> = HashMap::new();
    }

    fn parse(field_expression: &str, mut low: u8, mut high: u8) -> Result<Vec<u8>, ErrorCode> {
        let mut step = 1;
        let mut range = Vec::new();

        let expression = field_expression.split('/').collect::<Vec<&str>>();

        let has_range = expression[0].split('-').collect::<Vec<&str>>();

        if expression.len() == 2 {
            step = match expression[1].parse::<u8>() {
                Ok(low) => low,
                Err(err) => {
                    println!("{}", err);
                    return Err(ErrorCode::InvalidStep);
                }
            };
        }

        if has_range.len() == 2 {
            low = match has_range[0].parse::<u8>() {
                Ok(new_low) if new_low >= low && new_low <= high => new_low,
                Ok(_) => {
                    return Err(ErrorCode::InvalidRange);
                }
                Err(err) => {
                    println!("{}", err);
                    return Err(ErrorCode::InvalidRange);
                }
            };
            high = match has_range[1].parse::<u8>() {
                Ok(new_high) if new_high >= low && new_high <= high => new_high,
                Ok(_) => {
                    return Err(ErrorCode::InvalidRange);
                }
                Err(err) => {
                    println!("{}", err);
                    return Err(ErrorCode::InvalidRange);
                }
            };
            //println!("Has range  -> ({:2}, {:2})", low, high);
        } else if expression[0] != "*" {
            low = match expression[0].parse::<u8>() {
                Ok(low) => low,
                Err(err) => {
                    println!("{}", err);
                    return Err(ErrorCode::InvalidValue);
                }
            };
            high = low;
        }

        let mut i = low;
        while i <= high {
            if i == low {
                range.push(i);
                i += step;
            }
            low += 1;
        }

        //println!("Computed range -> {:?}", range);
        Ok(range)
    }
}

#[derive(Clone)]
struct CronField {
    name: CronFieldName,
    index: usize,
    range: Vec<u8>,
}

impl CronField {
    fn new(expression: &'static str, name: CronFieldName, low: u8, high: u8) -> CronField {
        CronField {
            name,
            index: 0,
            range: CronExpression::parse(expression, low, high).unwrap(),
        }
    }

    fn name(&self) -> CronFieldName {
        self.name
    }

    fn index(&self) -> usize {
        self.index
    }

    fn next(&mut self) -> u8 {
        self.index += 1;
        self.range[self.index]
    }
}

impl std::fmt::Debug for CronField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CronField {{
name        : {:?},
index       : {:?},
range       : {:?}, 
}}",
            self.name, self.index, self.range
        )
    }
}

impl std::fmt::Display for CronField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CronField {{
name        : {:?},
index       : {},
range       : {:?}, 
}}",
            self.name, self.index, self.range
        )
    }
}

#[derive(Clone)]
struct CronScheduleNg {
    minute: CronField,
    hour: CronField,
    day_of_month: CronField,
    month: CronField,
    day_of_week: CronField,
}

impl CronScheduleNg {
    fn new(cron_expression: &'static str) -> CronScheduleNg {
        let cron_fields = cron_expression.split(" ").collect::<Vec<&str>>();
        CronScheduleNg {
            minute: CronField::new(cron_fields[0], CronFieldName::Minute, 0, 59),
            hour: CronField::new(cron_fields[1], CronFieldName::Hour, 0, 23),
            day_of_month: CronField::new(cron_fields[2], CronFieldName::DayOfMonth, 1, 31),
            month: CronField::new(cron_fields[3], CronFieldName::Month, 1, 12),
            day_of_week: CronField::new(cron_fields[4], CronFieldName::DayOfWeek, 0, 7),
        }
    }

    fn get_schedule(&self, d: Datetime) {
        let mut new_cron_schedule = self.clone();

        let minute = *self
            .minute
            .range
            .get(CronScheduleNg::get_index(
                self.minute.range.clone(),
                d.minute,
            ))
            .unwrap();
        let hour = *self
            .hour
            .range
            .get(CronScheduleNg::get_index(self.hour.range.clone(), d.hour))
            .unwrap();
        let day_of_month = *self
            .day_of_month
            .range
            .get(CronScheduleNg::get_index(
                self.day_of_month.range.clone(),
                d.day_of_month,
            ))
            .unwrap();
        let month = *self
            .month
            .range
            .get(CronScheduleNg::get_index(self.month.range.clone(), d.month))
            .unwrap();

        let mut now_year = Utc::now().year();
        //"1996-12-19T16:39:57-08:00"
        let date_time = format!(
            "{now_year:0>4}-{month:0>2}-{day_of_month:0>2}T{hour:0>2}:{minute:0>2}:00+00:00"
        );

        println!("String -> {}", date_time);
        let date_time = DateTime::parse_from_rfc3339(date_time.as_str()).unwrap();
        if date_time.weekday().num_days_from_sunday() as u8 == d.day_of_week {
            println!("Valid schedule");
        }
        println!("{:?}", date_time);

        /*
        new_cron_schedule.minute.index = self.get_index(self.minute.range.clone(), d.minute);
        new_cron_schedule.hour.index = self.get_index(self.hour.range.clone(), d.hour);
        new_cron_schedule.day_of_month.index =
            self.get_index(self.day_of_month.range.clone(), d.day_of_month);
        new_cron_schedule.month.index = self.get_index(self.month.range.clone(), d.month);
        */
    }

    fn can_run_at(&self, d: Datetime) -> bool {
        self.minute.range.contains(&d.minute)
            & self.hour.range.contains(&d.hour)
            & self.day_of_month.range.contains(&d.day_of_month)
            & self.month.range.contains(&d.month)
            & self.day_of_week.range.contains(&d.day_of_week)
    }

    fn get_index(range: Vec<u8>, val: u8) -> usize {
        let mut k: usize = 0;
        let len = range.len();
        for &v in &range {
            println!(" --> {} {} {:?}", k, val, range);
            if v >= val {
                break;
            }
            k += 1;
        }
        k % len
    }

    fn next_runs() {
        let now = Datetime::new();
        let schedules: Vec<Datetime> = Vec::new();
    }
}

impl std::fmt::Debug for CronScheduleNg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CronScheduleNg {{
    minute      : {:?},
    hours       : {:?},
    day_of_month: {:?}, 
    month       : {:?}, 
    day_of_week : {:?},
}}",
            self.minute, self.hour, self.day_of_month, self.month, self.day_of_week
        )
    }
}
impl std::fmt::Display for CronScheduleNg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CronScheduleNg {{
    minute      : {:?},
    hours       : {:?},
    day_of_month: {:?}, 
    month       : {:?}, 
    day_of_week : {:?},
}}",
            self.minute, self.hour, self.day_of_month, self.month, self.day_of_week
        )
    }
}

#[derive(PartialEq)]
struct CronSchedule {
    minute: Vec<u8>,
    hour: Vec<u8>,
    day_of_month: Vec<u8>,
    month: Vec<u8>,
    day_of_week: Vec<u8>,
}

impl CronSchedule {
    fn new(cron_expression: &str) -> CronSchedule {
        //let cron_expression = "03-15/5,45-50,*/10 */4 1-7 12 1-4";
        let cron_fields = cron_expression.split(" ").collect::<Vec<&str>>();
        if cron_fields.len() != 5 {
            panic!("Invalid cron expression");
        }
        let (minute, hour, day_of_month, month, day_of_week) = (
            CronSchedule::process_field(cron_fields[0], 00, 59),
            CronSchedule::process_field(cron_fields[1], 00, 23),
            CronSchedule::process_field(cron_fields[2], 01, 31),
            CronSchedule::process_field(cron_fields[3], 01, 12),
            CronSchedule::process_field(cron_fields[4], 00, 07),
        );
        CronSchedule {
            minute,
            hour,
            day_of_month,
            month,
            day_of_week,
        }
    }

    fn parse_range(ch: &str, mut low: u8, mut high: u8) -> Result<Vec<u8>, ErrorCode> {
        let mut step = 1;
        let mut range = Vec::new();

        let expression = ch.split('/').collect::<Vec<&str>>();

        let has_range = expression[0].split('-').collect::<Vec<&str>>();

        if expression.len() == 2 {
            step = match expression[1].parse::<u8>() {
                Ok(low) => low,
                Err(err) => {
                    println!("{}", err);
                    return Err(ErrorCode::InvalidStep);
                }
            };
        }

        if has_range.len() == 2 {
            low = match has_range[0].parse::<u8>() {
                Ok(new_low) if new_low >= low && new_low <= high => new_low,
                Ok(_) => {
                    return Err(ErrorCode::InvalidRange);
                }
                Err(err) => {
                    println!("{}", err);
                    return Err(ErrorCode::InvalidRange);
                }
            };
            high = match has_range[1].parse::<u8>() {
                Ok(new_high) if new_high >= low && new_high <= high => new_high,
                Ok(_) => {
                    return Err(ErrorCode::InvalidRange);
                }
                Err(err) => {
                    println!("{}", err);
                    return Err(ErrorCode::InvalidRange);
                }
            };
            //println!("Has range  -> ({:2}, {:2})", low, high);
        } else if expression[0] != "*" {
            low = match expression[0].parse::<u8>() {
                Ok(low) => low,
                Err(err) => {
                    println!("{}", err);
                    return Err(ErrorCode::InvalidValue);
                }
            };
            high = low;
        }

        let mut i = low;
        while i <= high {
            if i == low {
                range.push(i);
                i += step;
            }
            low += 1;
        }

        //println!("Computed range -> {:?}", range);
        Ok(range)
    }

    fn get_list(ch: &str) -> Vec<&str> {
        ch.split(',').collect::<Vec<&str>>()
    }

    fn process_field(field: &str, low: u8, high: u8) -> Vec<u8> {
        let mut ranges = Vec::new();
        for expr in CronSchedule::get_list(field) {
            //println!("Expression -> {}", expr);
            let mut range = match CronSchedule::parse_range(expr, low, high) {
                Ok(range) => range,
                Err(err) => panic!("Error processing expression {} {:?}", expr, err),
            };
            ranges.append(&mut range);
        }
        ranges.sort();
        ranges.dedup();
        ranges
    }

    fn can_run_at(&self, d: Datetime) -> bool {
        self.minute.contains(&d.minute)
            & self.hour.contains(&d.hour)
            & self.day_of_month.contains(&d.day_of_month)
            & self.month.contains(&d.month)
            & self.day_of_week.contains(&d.day_of_week)
    }
}

impl std::fmt::Debug for CronSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CronSchedule {{
minute      : {:?},
hours       : {:?},
day_of_month: {:?}, 
month       : {:?}, 
day_of_week : {:?},
}}",
            self.minute, self.hour, self.day_of_month, self.month, self.day_of_week
        )
    }
}
impl std::fmt::Display for CronSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CronSchedule {{
minute      : {:?},
hours       : {:?},
day_of_month: {:?}, 
month       : {:?}, 
day_of_week : {:?},
}}",
            self.minute, self.hour, self.day_of_month, self.month, self.day_of_week
        )
    }
}

#[derive(Clone)]
struct Datetime {
    utc_now: DateTime<Utc>,
    minute: u8,
    hour: u8,
    day_of_month: u8,
    month: u8,
    day_of_week: u8,
}

impl Datetime {
    fn new() -> Datetime {
        let utc_now = Utc::now();
        Datetime {
            utc_now,
            minute: utc_now.minute() as u8,
            hour: utc_now.hour() as u8,
            day_of_month: utc_now.day() as u8,
            month: utc_now.month() as u8,
            day_of_week: utc_now.weekday().num_days_from_sunday() as u8,
        }
    }

    fn add_seconds(&self, seconds: i64) -> Datetime {
        let utc_now = self.utc_now.add(chrono::Duration::seconds(seconds));
        Datetime {
            utc_now,
            minute: utc_now.minute() as u8,
            hour: utc_now.hour() as u8,
            day_of_month: utc_now.day() as u8,
            month: utc_now.month() as u8,
            day_of_week: utc_now.weekday().num_days_from_sunday() as u8,
        }
    }

    fn from_string(date_time: &str) {
        //let date_time = "2020-04-12 22:10:57";
        let utc_now = DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00");
    }
}

impl std::fmt::Display for Datetime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DateTime {{
minute      : {:?},
hours       : {:?},
day_of_month: {:?}, 
month       : {:?}, 
day_of_week : {:?},
}}",
            self.minute, self.hour, self.day_of_month, self.month, self.day_of_week,
        )
    }
}

fn main() {
    //cron_expression format
    //minute hours day_of_month month day_of_week
    let cron_expression = "*/10 */6 * * *";
    //let cron_expression = "03-15/5,45-50,*/10 1-23/2 2 * *";
    let cron_schedule = CronSchedule::new(cron_expression);
    let cron_schedule_ng = CronScheduleNg::new(cron_expression);
    println!(
        "{}
Parsed -> {}
Ng     -> {:?}
",
        cron_expression, cron_schedule, cron_schedule_ng,
    );

    let now = Datetime::new();
    let future = now.add_seconds(60);
    println!("Current time -> {}", now);
    println!(
        "Ready to run in future -> {}\n{}",
        future,
        cron_schedule_ng.can_run_at(future.clone()),
    );
    cron_schedule_ng.get_schedule(now);
}

#[test]
fn test_evaluate_cron_expression() {
    let cron_expression = "*/10 */6 1 * */2";
    let cron_schedule = CronSchedule::new(cron_expression);
    assert_eq!(
        cron_schedule,
        CronSchedule {
            minute: vec![0, 10, 20, 30, 40, 50],
            hour: vec![0, 6, 12, 18],
            day_of_month: vec![1],
            month: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            day_of_week: vec![0, 2, 4, 6],
        }
    )
}

#[test]
#[should_panic]
fn test_evaluate_invalid_cron_expression() {
    let cron_expression = "A/10 */6 1 * */2";
    CronSchedule::new(cron_expression);
}
