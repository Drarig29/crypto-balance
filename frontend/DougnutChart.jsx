import React from 'react';

import Rainbow from 'rainbowvis.js';

import { Legend } from 'recharts';
import { Tooltip } from 'recharts';
import { PieChart } from 'recharts';
import { Pie } from 'recharts';
import { Cell } from 'recharts';

export default function ({ data }) {
    const assets = (Object.keys(data || {})).filter(key => key !== "Total as BTC" && key !== "time");
    console.log({ assets });

    const values = assets.map(asset => ({ name: asset, value: data[asset] }));
    console.log({ values });

    const rainbow = new Rainbow();
    rainbow.setNumberRange(0, values.length + 1);

    return (
        <PieChart width={300} height={300} margin={{ left: 50, right: 50 }}>
            <Pie data={values} nameKey="name" dataKey="value" cx="50%" cy="50%" innerRadius={50} paddingAngle={5}>
                {values.map((_, index) => {
                    const color = `#${rainbow.colorAt(index)}`;
                    return <Cell key={index} stroke={color} fill={color} fillOpacity={0.6} />
                })}
            </Pie>
            <Tooltip />
            <Legend />
        </PieChart>
    )
}