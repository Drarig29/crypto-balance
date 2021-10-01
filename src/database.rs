use crate::aggregate::make_aggregate_query;
use crate::model::database;
use crate::rocket::futures::StreamExt;

use bson::Bson;
use bson::Document;
use chrono::{DateTime, Utc};
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use mongodb::Collection;
use mongodb::Database;
use std::iter::Iterator;

pub async fn get_snapshots(
    database: &Database,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Vec<database::Snapshot> {
    let collection: Collection<Document> = database.collection("snapshots");

    // Sort with older first.
    let find_options = FindOptions::builder().sort(doc! {"time": 1}).build();

    collection
        .find(
            doc! {
                "time": {
                    "$gte": start,
                    "$lte": end,
                }
            },
            find_options,
        )
        .await
        .unwrap()
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .flatten()
        .map(|document| bson::from_bson(Bson::Document(document)).unwrap())
        .collect()
}

pub async fn push_snapshots(database: &Database, snapshots: Vec<database::Snapshot>) {
    let collection: Collection<Document> = database.collection("snapshots");

    let docs: Vec<bson::Document> = snapshots
        .iter()
        .map(|history| bson::ser::to_document(history).unwrap())
        .collect();

    let res = collection.insert_many(docs, None).await;
    res.expect("Could not insert the snapshots.");
}

pub async fn get_possessed_assets(database: &Database) -> Vec<String> {
    let collection: Collection<Document> = database.collection("snapshots");

    let mut assets: Vec<String> = collection
        .distinct("balances.asset", None, None)
        .await
        .unwrap()
        .iter()
        .map(|document| bson::from_bson(document.to_owned()).unwrap())
        .collect();

    let bitcoin_asset = "BTC".to_string();

    if !assets.contains(&bitcoin_asset) {
        assets.push(bitcoin_asset);
    }

    assets.sort_unstable();
    assets
}

pub async fn push_history(database: &Database, price_history: Vec<database::CurrencyHistory>) {
    let collection: Collection<Document> = database.collection("history");

    let docs: Vec<bson::Document> = price_history
        .iter()
        .map(|history| bson::ser::to_document(history).unwrap())
        .collect();

    let res = collection.insert_many(docs, None).await;
    res.expect("Could not insert the history.");
}

pub async fn get_computed_snapshots(
    database: &Database,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Vec<database::ComputedSnapshot> {
    let collection: Collection<Document> = database.collection("snapshots");

    let computed_snapshots: Vec<database::ComputedSnapshot> = collection
        .aggregate(make_aggregate_query(start, end), None)
        .await
        .unwrap()
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .flatten()
        .map(|document| bson::from_bson(Bson::Document(document)).unwrap())
        .collect();

    computed_snapshots
}
