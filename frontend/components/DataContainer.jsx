import moment from 'moment';
import React, { useContext, useEffect, useState } from 'react';

import { AreaChart } from './AreaChart';
import { DatePicker } from './DatePicker';
import { DonutChart } from './DonutChart';
import { VisibilityButton } from './VisibilityButton';

import { sendRequest, toCurrency, toDateString, toISOString } from '../helpers';
import { Context } from '..';

function Spinner({ visible }) {
    return <span className="spinner" style={{ display: visible ? 'block' : 'none' }}></span>;
}

function LogoutButton({ onLogout }) {
    return (
        <a className='logout-btn' onClick={onLogout}>
            <img src='assets/logout.svg'></img>
        </a>
    );
}

function transformData(snapshots) {
    const transformed = snapshots.map(snapshot => Object.assign({
        time: toDateString(snapshot.time),
        'Total as BTC': snapshot.total_asset_of_btc,
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
        setLoading(true);
        setSelectedIndex(null);

        const getSnapshots = async () => {
            const response = await sendRequest('/api', {
                password: context.password,
                conversion: context.currency.name,
                start: toISOString(dateRange.from),
                end: toISOString(dateRange.to),
            });

            if (response.status !== 200) {
                throw response.body;
            }

            return transformData(response.body);
        };

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
    };

    const handleLogout = () => {
        setContext({
            ...context,
            password: null,
        });
    };

    const currentSnapshot = snapshots[selectedIndex !== null ? selectedIndex : snapshots.length - 1];

    return (
        <main>
            <header>
                <div>
                    <DatePicker initialRange={dateRange} onRangeChange={(from, to) => setDateRange({ from, to })} />
                    <Spinner visible={loading} />
                </div>
                <aside>
                    <LogoutButton onLogout={handleLogout} />
                    Total (BTC) : {currentSnapshot && toCurrency(currentSnapshot['Total as BTC'].value, context)}
                    <VisibilityButton initiallyRevealed={context.revealValues} onRevealedChange={handleRevealedChange} />
                </aside>
            </header>
            <AreaChart data={snapshots} onDateClicked={index => setSelectedIndex(index)} />
            <DonutChart data={currentSnapshot} label={currentSnapshot && currentSnapshot.time} />
        </main>
    );
};