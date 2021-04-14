import moment from 'moment';
import React, { useEffect, useState } from 'react';

import AreaChart from './AreaChart';
import DatePicker from './DatePicker';
import DougnutChart from './DougnutChart';
import { toISOString } from './helpers';

const currency = {
    name: 'EUR',
    symbol: '€',
}

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

export default function () {
    const [snapshots, setSnapshots] = useState([]);

    const [dateRange, setDateRange] = useState({
        from: moment().subtract(1, 'month').toDate(),
        to: moment().toDate(),
    });

    const [loading, setLoading] = useState(false);
    const [selectedIndex, setSelectedIndex] = useState(null);

    useEffect(() => {
        console.log(dateRange);

        const body = JSON.stringify({
            conversion: currency.name,
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
        setSelectedIndex(null);

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
            <AreaChart currency={currency.symbol} data={snapshots} onDateClicked={index => setSelectedIndex(index)} />
            <DougnutChart currency={currency.symbol} data={snapshots[selectedIndex !== null ? selectedIndex : snapshots.length - 1]} />
        </>
    )
}