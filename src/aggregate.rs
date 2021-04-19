use bson::Bson;
use chrono::{DateTime, Utc};
use mongodb::bson;
use mongodb::bson::doc;

pub fn make_aggregate_query(start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<bson::Document> {
    vec![
        doc! {
          "$match": {
            "time": {
              "$gte": Bson::DateTime(start),
              "$lte": Bson::DateTime(end)
            }
          }
        },
        doc! {
          "$sort": {
            "time": 1
          }
        },
        doc! {
          "$lookup": {
            "from": "history",
            "localField": "time",
            "foreignField": "time",
            "as": "prices"
          }
        },
        doc! {
          "$project": {
            "time": 1,
            "total_asset_of_btc": compute_value("BTC", "$total_asset_of_btc"),
            "balances": {
              "$map": {
                "input": "$balances",
                "as": "balance",
                "in": compute_value("$$balance.asset", "$$balance.amount")
              }
            }
          }
        },
    ]
}

fn compute_value(asset_identifier: &str, amount_identifier: &str) -> bson::Document {
    doc! {
      "$mergeObjects": [
        {
          "asset": asset_identifier,
          "amount": amount_identifier,
        },
        {
          "$let": {
            "vars": {
              "found": {
                "$first": {
                  "$filter": {
                    "input": "$prices",
                    "as": "price",
                    "cond": {
                      "$eq": ["$$price.asset", asset_identifier]
                    }
                  }
                }
              }
            },
            "in": {
              "price": "$$found.price",
              "value": { "$multiply": [amount_identifier, "$$found.price"] }
            }
          }
        }
      ]
    }
}
