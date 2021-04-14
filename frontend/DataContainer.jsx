import React, { useEffect, useState } from 'react';
import ReactDOM from 'react-dom';

import AreaChart from './AreaChart';

function transformData(snapshots) {
    const transformed = snapshots.map(snapshot => Object.assign({
        time: new Date(snapshot.time.$date).toDateString(),
        "Total as BTC": snapshot.total_asset_of_btc.value,
    }, ...snapshot.balances.map(balance => ({
        [balance.asset]: balance.value.toFixed(2)
    }))));

    console.log(transformed);
    return transformed;
}


export default function () {
    const [snapshots, setSnapshots] = useState([]);

    const [start, setStart] = useState("2021-04-01T00:00:00Z");
    const [end, setEnd] = useState("2021-04-12T00:00:00Z");

    useEffect(() => {
        const body = JSON.stringify({
            "conversion": "EUR",
            start,
            end,
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