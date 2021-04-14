import moment from 'moment';
import React, { useEffect, useState } from 'react';

import AreaChart from './AreaChart';
import DatePicker from './DatePicker';
import DougnutChart from './DougnutChart';

function Spinner({ visible }) {
    return <span className="spinner" style={{ visibility: visible ? 'visible' : 'hidden' }}></span>
}

function transformData(snapshots) {
    const transformed = snapshots.map(snapshot => Object.assign({
        time: new Date(snapshot.time.$date).toDateString(),
        "Total as BTC": snapshot.total_asset_of_btc.value,
    }, ...snapshot.balances.map(balance => ({
        [balance.asset]: balance.value
    }))));

    console.log({ snapshots, transformed });
    return transformed;
}

function toISOString(date) {
    return date.toISOString().split('T')[0] + 'T00:00:00Z';
}

export default function () {
    const [snapshots, setSnapshots] = useState([]);

    const [dateRange, setDateRange] = useState({
        from: moment().subtract(1, 'month').toDate(),
        to: moment().toDate(),
    });

    const [loading, setLoading] = useState(false);

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

        setLoading(true);

        fetch("http://127.0.0.1:8000/api", options)
            .then(response => response.json())
            .then(snapshots => {
                setLoading(false);
                setSnapshots(transformData(snapshots));
            })
            .catch(error => {
                setLoading(false);
                alert(error);
            });
    }, [dateRange]);

    return (
        <>
            <header>
                <DatePicker initialRange={dateRange} onRangeChange={(from, to) => setDateRange({ from, to })} />
                <Spinner visible={loading} />
            </header>
            <AreaChart data={snapshots} />
            <DougnutChart data={snapshots[snapshots.length - 1]} />
        </>
    )
}