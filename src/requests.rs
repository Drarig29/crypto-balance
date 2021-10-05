use crate::model::{binance, database, nomics};
use crate::utils;
use crate::Environment;
use crate::TimeSpan;
use crate::{BINANCE_API_BASE_URL, NOMICS_API_BASE_URL};
use std::error::Error;

use chrono::{DateTime, Duration, TimeZone, Utc};
use reqwest::Client;
use std::time::SystemTime;

pub async fn get_all_snapshots(
    auth: &Environment,
    account_type: &str,
    limit: u8,
    timespans: &[TimeSpan],
) -> Result<Vec<database::Snapshot>, Box<dyn Error>> {
    let mut snapshots = Vec::new();

    for timespan in timespans {
        let intermediate_results =
            get_snapshots(auth, account_type, limit, timespan.start, timespan.end).await;

        let mut intermediate_results = match intermediate_results {
            Ok(intermediate_results) => intermediate_results,
            Err(e) => return Err(e),
        };

        snapshots.append(&mut intermediate_results);
    }

    Ok(snapshots)
}

pub async fn get_all_history(
    auth: &Environment,
    ids: &[String],
    convert: &str,
    timespans: &[TimeSpan],
) -> Result<Vec<database::CurrencyHistory>, Box<dyn Error>> {
    let mut snapshots = Vec::new();

    for timespan in timespans {
        let mut intermediate_results =
            match get_history(auth, ids, convert, timespan.start, timespan.end).await {
                Ok(intermediate_results) => intermediate_results,
                Err(e) => return Err(e),
            };

        snapshots.append(&mut intermediate_results);
    }

    Ok(snapshots)
}

async fn get_snapshots(
    auth: &Environment,
    account_type: &str,
    limit: u8,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<database::Snapshot>, Box<dyn Error>> {
    let client = Client::new();

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let shifted_start = if start == end {
        start - Duration::days(1)
    } else {
        start - Duration::seconds(1)
    };

    let shifted_end = end - Duration::seconds(1);

    println!(
        "Call Binance API (start: {}, end: {})",
        shifted_start, shifted_end
    );

    let params = format!(
        "type={}&limit={}&timestamp={}&startTime={}&endTime={}",
        account_type,
        limit,
        now,
        shifted_start.timestamp_millis(),
        shifted_end.timestamp_millis(),
    );

    let signature = utils::get_mac_sha256(&params, &auth.binance_secret);

    let url = format!(
        "{}?{}&signature={}",
        BINANCE_API_BASE_URL, params, signature
    );
    let res = client
        .get(url)
        .header("X-MBX-APIKEY", auth.binance_key.to_owned())
        .send()
        .await?;

    let json = match res.text().await {
        Ok(res) => res,
        Err(e) => return Err(Box::new(e)),
    };

    let obj = match serde_json::from_str::<binance::RootObject>(&json) {
        Ok(obj) => obj,
        Err(e) => return Err(Box::new(e)),
    };

    let snapshots: Vec<database::Snapshot> = obj
        .snapshots
        .iter()
        .map(|snapshot| database::Snapshot {
            time: chrono::Utc.timestamp_millis(snapshot.update_time) + Duration::seconds(1),
            balances: snapshot
                .data
                .balances
                .iter()
                .filter(|balance| balance.free.parse::<f32>().unwrap() > 0.)
                .map(|balance| database::Balance {
                    asset: balance.asset.to_owned(),
                    amount: balance.free.parse::<f32>().unwrap(),
                })
                .collect(),
            total_asset_of_btc: snapshot.data.total_asset_of_btc.parse::<f32>().unwrap(),
        })
        .collect();

    println!("Got {} snapshots.", snapshots.len());

    Ok(snapshots)
}

async fn get_history(
    auth: &Environment,
    ids: &[String],
    convert: &str,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<database::CurrencyHistory>, Box<dyn Error>> {
    let client = Client::new();

    println!("Call Nomics API (start: {}, end: {})", start, end);

    let params = format!(
        "ids={}&convert={}&start={}&end={}",
        ids.join(","),
        convert,
        utils::get_uri_escaped_datetime(start),
        utils::get_uri_escaped_datetime(end),
    );

    let url = format!("{}?key={}&{}", NOMICS_API_BASE_URL, auth.nomics_key, params);
    let res = client.get(url).send().await?;

    let json = match res.text().await {
        Ok(res) => res,
        Err(e) => return Err(Box::new(e)),
    };

    let obj = match serde_json::from_str::<Vec<nomics::Sparkline>>(&json) {
        Ok(obj) => obj,
        Err(e) => return Err(Box::new(e)),
    };

    let history: Vec<database::CurrencyHistory> = obj
        .iter()
        .map(|history| {
            history
                .timestamps
                .iter()
                .enumerate()
                .map(|(i, timestamp)| database::CurrencyHistory {
                    asset: history.currency.to_owned(),
                    time: DateTime::parse_from_rfc3339(timestamp)
                        .unwrap()
                        .with_timezone(&Utc),
                    price: history.prices[i].parse::<f32>().unwrap(),
                })
                .collect::<Vec<database::CurrencyHistory>>()
        })
        .flatten()
        .collect();

    println!("Got {} currencies history.", history.len());

    Ok(history)
}
