import React, { useEffect, useState } from 'react';
import ReactDOM from 'react-dom';

import AreaChart from './AreaChart';

function transformData(snapshots) {
    const transformed = snapshots.map(snapshot => Object.assign({
        time: snapshot.time.$date,
        "Total as BTC": snapshot.total_asset_of_btc.value,
    }, ...snapshot.balances.map(balance => ({
        [balance.asset]: balance.value
    }))));

    console.log(transformed);
    return transformed;
}

export default function () {
    const [snapshots, setSnapshots] = useState([]);

    useEffect(() => {
        const body = JSON.stringify({
            "conversion": "EUR",
            "start": "2021-04-08T00:00:00Z",
            "end": "2021-04-12T00:00:00Z"
        });

        const headers = new Headers();
        headers.append("Content-Type", "application/json");

        const options = {
            method: 'POST',
            headers,
            body,
        };

        fetch("http://127.0.0.1:8000/api", options)
            .then(response => response.json())
            .then(snapshots => setSnapshots(transformData(snapshots)))
            .catch(error => alert(error));
    }, []);

    return (
        <div>
            <p>Data container</p>
            <AreaChart data={snapshots} />
        </div>
    )
}