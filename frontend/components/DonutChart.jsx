import React, { useContext, useState } from 'react';

import Rainbow from 'rainbowvis.js';
import { Legend, Tooltip, PieChart, Pie, Cell, ResponsiveContainer } from 'recharts';

import { toCurrency } from '../helpers';
import { Context } from '..';

export const DonutChart = ({ label, data }) => {
    const [context] = useContext(Context);
    const [showLabel, setShowLabel] = useState(false);

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
                <Pie animationDuration={1000} onAnimationEnd={() => setShowLabel(true)} data={values} nameKey="name" dataKey="value" cx="50%" cy="50%" innerRadius={75} paddingAngle={5}>
                    {values.map((value, index) => {
                        const color = `#${rainbow.colorAt(index)}`;
                        return <Cell key={index} stroke={color} fill={color} fillOpacity={0.6} strokeWidth={thickness[value.name]} />
                    })}
                </Pie>
                <Tooltip formatter={value => toCurrency(value, context)} />
                <Legend onMouseEnter={handleMouseEnter} onMouseLeave={handleMouseLeave} />
                {showLabel && <text className="recharts-text" x="50%" y="50%" textAnchor="middle" dy={-6} fill="white">
                    {label}
                </text>}
            </PieChart>
        </ResponsiveContainer>
    )
}