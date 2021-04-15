import moment from 'moment';
import React, { useContext, useEffect, useState } from 'react';

import { AreaChart } from './AreaChart';
import { DatePicker } from './DatePicker';
import { DonutChart } from './DonutChart';
import { VisibilityButton } from './VisibilityButton';

import { toCurrency, toISOString } from '../helpers';
import { Context } from '..';

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

export const DataContainer = () => {
    const [context, setContext] = useContext(Context);

    const [snapshots, setSnapshots] = useState([]);
    const [loading, setLoading] = useState(false);
    const [selectedIndex, setSelectedIndex] = useState(null);

    const [dateRange, setDateRange] = useState({
        from: moment().subtract(1, 'month').toDate(),
        to: moment().toDate(),
    });

    useEffect(() => {
        console.log(dateRange);

        const body = JSON.stringify({
            conversion: context.currency.name,
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

        fetch("/api", options)
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

    const handleRevealedChange = revealed => {
        setContext({
            ...context,
            revealValues: revealed,
        });
    }

    const currentSnapshot = snapshots[selectedIndex !== null ? selectedIndex : snapshots.length - 1];

    return (
        <>
            <header>
                <DatePicker initialRange={dateRange} onRangeChange={(from, to) => setDateRange({ from, to })} />
                <Spinner visible={loading} />
                <aside>
                    <VisibilityButton initiallyRevealed={context.revealValues} onRevealedChange={handleRevealedChange} />
                    Total: {currentSnapshot && toCurrency(currentSnapshot['Total as BTC'], context)}
                </aside>
            </header>
            <AreaChart data={snapshots} onDateClicked={index => setSelectedIndex(index)} />
            <DonutChart data={currentSnapshot} />
        </>
    )
}