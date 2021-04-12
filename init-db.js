//TODO: make compound index with time and asset and check if it is used by the aggregation pipeline

db.snapshots.createIndex({ time: 1 });
db.history.createIndex({ time: 1 });