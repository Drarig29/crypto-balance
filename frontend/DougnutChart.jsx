import React from 'react';
import ReactDOM from 'react-dom';

import { PieChart } from 'recharts';
import { Pie } from 'recharts';

export default function ({ data }) {
    const assets = (Object.keys(data || {})).filter(key => key !== "Total as BTC" && key !== "time");
    console.log({ assets });

    const values = assets.map(asset => ({ name: asset, value: data[asset] }));
    console.log({ values });

    return (
        <PieChart width={300} height={300} margin={{ left: 50, right: 50 }}>
            <Pie data={values} nameKey="name" dataKey="value" cx="50%" cy="50%" fill="#82ca9d" label></Pie>
        </PieChart>
    )
}