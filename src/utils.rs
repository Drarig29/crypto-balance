use crate::model::database;
use crate::TimeSpan;

use chrono::{DateTime, Duration, SecondsFormat, Utc};

use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn get_missing_timespans(needed: TimeSpan, available: TimeSpan) -> Vec<TimeSpan> {
    assert!(needed.start <= needed.end);
    assert!(available.start <= available.end);

    if needed.start >= available.start && needed.end <= available.end {
        return vec![];
    }

    if needed.start >= available.end && needed.end >= available.end {
        let start = if needed.start == available.end {
            needed.start + Duration::days(1)
        } else {
            needed.start
        };

        return vec![TimeSpan { start, ..needed }];
    }

    if needed.start <= available.start && needed.end <= available.start {
        let end = if needed.end == available.start {
            needed.end - Duration::days(1)
        } else {
            needed.end
        };

        return vec![TimeSpan { end, ..needed }];
    }

    if needed.start <= available.start && needed.end <= available.end {
        let end = available.start - Duration::days(1);
        return vec![TimeSpan { end, ..needed }];
    }

    if needed.start >= available.start && needed.end >= available.end {
        let start = available.end + Duration::days(1);
        return vec![TimeSpan { start, ..needed }];
    }

    if needed.start <= available.start && needed.end >= available.end {
        let end = available.start - Duration::days(1);
        let start = available.end + Duration::days(1);
        return vec![TimeSpan { end, ..needed }, TimeSpan { start, ..needed }];
    }

    panic!("Unsupported case!");
}

pub fn get_timespans_to_retrieve(
    snapshots: Vec<database::Snapshot>,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Vec<TimeSpan> {
    if snapshots.is_empty() {
        println!("No available data.");
        return vec![TimeSpan { start, end }];
    }

    let database_start: DateTime<Utc> = snapshots.first().unwrap().time;
    let database_end: DateTime<Utc> = snapshots.last().unwrap().time;
    println!(
        "Database start: {}\nDatabase end: {}",
        database_start, database_end
    );

    let needed = TimeSpan { start, end };

    let available = TimeSpan {
        start: database_start,
        end: database_end,
    };

    let missing = get_missing_timespans(needed, available);

    println!("Missing: {:?}", missing);

    missing
}

pub fn split_timespan_max_days(timespan: &TimeSpan, max_days: i64) -> Vec<TimeSpan> {
    if (timespan.end - timespan.start).num_days() < max_days {
        return vec![TimeSpan {
            start: timespan.start,
            end: timespan.end,
        }];
    }

    let mut timespans: Vec<TimeSpan> = vec![];

    let mut current_start = timespan.start;
    let mut current_end = timespan.start + Duration::days(max_days - 1);

    while current_end < timespan.end {
        timespans.push(TimeSpan {
            start: current_start,
            end: current_end,
        });

        current_start = current_end + Duration::days(1);
        current_end = current_start + Duration::days(max_days - 1);
    }

    timespans.push(TimeSpan {
        start: current_start,
        end: timespan.end,
    });

    timespans
}

pub fn split_all_timespans_max_days(timespans: &[TimeSpan], max_days: i64) -> Vec<TimeSpan> {
    let mut results: Vec<TimeSpan> = vec![];

    for timespan in timespans {
        let mut intermediate_results = split_timespan_max_days(timespan, max_days);
        results.append(&mut intermediate_results);
    }

    results
}

pub fn get_uri_escaped_datetime(datetime: DateTime<Utc>) -> String {
    let formatted = datetime.to_rfc3339_opts(SecondsFormat::Secs, true);
    formatted.replace(":", "%3A")
}

pub fn get_mac_sha256(data: &String, secret: &String) -> String {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(data.as_bytes());

    let hash_message = mac.finalize().into_bytes();
    hex::encode(&hash_message)
}
