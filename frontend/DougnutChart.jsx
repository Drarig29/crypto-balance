import React, { useState } from 'react';

import Rainbow from 'rainbowvis.js';

import { Legend } from 'recharts';
import { Tooltip } from 'recharts';
import { PieChart } from 'recharts';
import { Pie } from 'recharts';
import { Cell } from 'recharts';
import { ResponsiveContainer } from 'recharts';

export default function ({ data }) {
    const assets = (Object.keys(data || {})).filter(key => key !== "Total as BTC" && key !== "time");
    console.log({ assets });

    const values = assets.map(asset => ({ name: asset, value: data[asset] }));
    console.log({ values });

    const [thickness, setThickness] = useState(Object.fromEntries(assets.map(asset => [asset, 1])));

    const rainbow = new Rainbow();
    rainbow.setNumberRange(0, values.length + 1);

    const handleMouseEnter = (o) => {
        const { value } = o;
        setThickness({ ...thickness, [value]: 3 });
    };

    const handleMouseLeave = (o) => {
        const { value } = o;
        setThickness({ ...thickness, [value]: 1 });
    };

    return (
        <ResponsiveContainer width="60%" height={300} margin={{ left: 50, right: 50 }}>
            <PieChart>
                <Pie data={values} nameKey="name" dataKey="value" cx="50%" cy="50%" innerRadius={50} paddingAngle={5}>
                    {values.map((value, index) => {
                        const color = `#${rainbow.colorAt(index)}`;
                        return <Cell key={index} stroke={color} fill={color} fillOpacity={0.6} strokeWidth={thickness[value.name]} />
                    })}
                </Pie>
                <Tooltip />
                <Legend onMouseEnter={handleMouseEnter} onMouseLeave={handleMouseLeave} />
            </PieChart>
        </ResponsiveContainer>
    )
}