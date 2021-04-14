import React, { useEffect, useState } from 'react';
import ReactDOM from 'react-dom';

import AreaChart from './AreaChart';
import DatePicker from './DatePicker';

function transformData(snapshots) {
    const transformed = snapshots.map(snapshot => Object.assign({
        time: new Date(snapshot.time.$date).toDateString(),
        "Total as BTC": snapshot.total_asset_of_btc.value,
    }, ...snapshot.balances.map(balance => ({
        [balance.asset]: balance.value && balance.value.toFixed(2)
    }))));

    console.log({ transformed });
    return transformed;
}

function toISOString(date) {
    return date.toISOString().split('T')[0] + 'T00:00:00Z';
}

export default function () {
    const [snapshots, setSnapshots] = useState([]);

    const [dateRange, setDateRange] = useState({
        from: new Date("2021-04-01T00:00:00Z"),
        to: new Date("2021-04-12T00:00:00Z"),
    });

    useEffect(() => {
        console.log(dateRange);

        const body = JSON.stringify({
            conversion: "EUR",
            start: toISOString(dateRange.from),
            end: toISOString(dateRange.to),
        });

        console.log({ body });

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
    }, [dateRange]);

    return (
        <div>
            <p>Data container</p>
            <DatePicker onRangeChange={(from, to) => setDateRange({ from, to })} />
            <AreaChart data={snapshots} />
        </div>
    )
}