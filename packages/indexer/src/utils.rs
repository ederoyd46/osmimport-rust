use std::ops::Add;
use std::fmt::Display;
use chrono::{DateTime, NaiveDateTime, Utc};

pub const NANO: f64 = 1000000000.0;

pub fn delta_decode<T>(seed: T, data: &[T]) -> Vec<T>
where
    T: Add<Output = T> + Copy + Display,
{
    let mut decoded: Vec<T> = vec![];
    let mut running_total = seed;
    for e in data.iter() {
        running_total = running_total + *e;
        decoded.push(running_total);
    }
    decoded
}

pub fn get_datetime(timestamp: i64) -> DateTime<Utc> {
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
     DateTime::from_utc(naive, Utc)
}

pub fn calculate_degrees(coordinate: i64, granularity: f64) -> f64 {
    (coordinate as f64 * granularity) / NANO
}

#[test]
fn it_should_calculate_correct_degrees() {
    assert_eq!(calculate_degrees(10000000, 100.0), 1.0);
}
