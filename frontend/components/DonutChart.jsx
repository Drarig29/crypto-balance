import React, { useMemo, useState } from 'react';

import Rainbow from 'rainbowvis.js';
import { Legend, Tooltip, PieChart, Pie, Cell, ResponsiveContainer } from 'recharts';

import { CustomTooltip } from './CustomTooltip';

export const DonutChart = ({ label, data }) => {
    const total = data && data["Total as BTC"].value;
    const assets = useMemo(() => (Object.keys(data || {})).filter(key => key !== "Total as BTC" && key !== "time"), [data]);
    const values = useMemo(() => assets.map(asset => ({ name: asset, value: data[asset].value, percent: data[asset].value / total })), [assets]);

    const [thickness, setThickness] = useState(Object.fromEntries(assets.map(asset => [asset, 1])));

    const rainbow = new Rainbow();
    rainbow.setNumberRange(0, values.length + 1);

    const handleMouseEnter = (props) => {
        const { value } = props;
        setThickness({ ...thickness, [value]: 3 });
    };

    const handleMouseLeave = (props) => {
        const { value } = props;
        setThickness({ ...thickness, [value]: 1 });
    };

    const renderActiveShape = (props) => {
        const { cx, cy } = props;
        return (
            <g>
                <text className="recharts-text" x={cx} y={cy} dy={8} textAnchor="middle">{label}</text>
            </g>
        );
    }

    return (
        <ResponsiveContainer width="90%" height={300} margin={{ left: 50, right: 50 }}>
            <PieChart>
                <Pie animationDuration={1000} data={values}
                    nameKey="name" dataKey="value" cx="50%" cy="50%"
                    innerRadius={75} paddingAngle={5}
                    activeShape={renderActiveShape} activeIndex={0}>
                    {values.map((value, index) => {
                        const color = `#${rainbow.colorAt(index)}`;
                        return <Cell key={index} stroke={color} fill={color} fillOpacity={0.6} strokeWidth={thickness[value.name]} />
                    })}
                </Pie>
                <Tooltip content={CustomTooltip} />
                <Legend onMouseEnter={handleMouseEnter} onMouseLeave={handleMouseLeave} />
            </PieChart>
        </ResponsiveContainer>
    )
}