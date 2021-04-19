db.snapshots.createIndex({ time: 1 }, { unique: true });
db.history.createIndex({ time: 1, asset: 1 }, { unique: true });