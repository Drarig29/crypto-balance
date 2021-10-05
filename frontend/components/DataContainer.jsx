import moment from 'moment';
import React, { useContext, useEffect, useState } from 'react';

import { AreaChart } from './AreaChart';
import { DatePicker } from './DatePicker';
import { DonutChart } from './DonutChart';
import { VisibilityButton } from './VisibilityButton';

import { toCurrency, toDateString, toISOString } from '../helpers';
import { Context } from '..';

function Spinner({ visible }) {
    return <span className="spinner" style={{ display: visible ? 'block' : 'none' }}></span>
}

function transformData(snapshots) {
    const transformed = snapshots.map(snapshot => Object.assign({
        time: toDateString(snapshot.time),
        "Total as BTC": snapshot.total_asset_of_btc,
    }, ...snapshot.balances.map(balance => ({
        [balance.asset]: balance
    }))));

    console.debug({ snapshots, transformed });
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
        console.debug(dateRange);

        const body = JSON.stringify({
            conversion: context.currency.name,
            start: toISOString(dateRange.from),
            end: toISOString(dateRange.to),
        });

        console.debug({ body });

        const headers = new Headers();
        headers.append("Content-Type", "application/json");

        const options = {
            method: 'POST',
            headers,
            body,
        };

        setLoading(true);
        setSelectedIndex(null);

        const getSnapshots = async () => {
            const response = await fetch('/api', options);
            const obj = await response.json();

            if (response.status !== 200) {
                throw obj;
            }

            return transformData(obj);
        }

        getSnapshots()
            .then(snapshots => {
                setLoading(false);
                setSnapshots(snapshots);
            })
            .catch(e => {
                setLoading(false);
                alert(e);
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
                <div>
                    <DatePicker initialRange={dateRange} onRangeChange={(from, to) => setDateRange({ from, to })} />
                    <Spinner visible={loading} />
                </div>
                <aside>
                    Total (BTC) : {currentSnapshot && toCurrency(currentSnapshot['Total as BTC'].value, context)}
                    <VisibilityButton initiallyRevealed={context.revealValues} onRevealedChange={handleRevealedChange} />
                </aside>
            </header>
            <AreaChart data={snapshots} onDateClicked={index => setSelectedIndex(index)} />
            <DonutChart data={currentSnapshot} label={currentSnapshot && currentSnapshot.time} />
        </>
    )
}