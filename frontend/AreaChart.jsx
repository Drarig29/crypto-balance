import React, { useState } from 'react';

import Rainbow from 'rainbowvis.js';
import Checkbox from './CheckBox';

import { YAxis } from 'recharts';
import { Tooltip } from 'recharts';
import { Area } from 'recharts';
import { CartesianGrid } from 'recharts';
import { XAxis } from 'recharts';
import { AreaChart } from 'recharts';
import { Legend } from 'recharts';
import { ResponsiveContainer } from 'recharts';

export default function ({ data, onDateClicked }) {
    const assets = (data.length > 0 && Object.keys(data[0]) || []).filter(key => key !== "Total as BTC" && key !== "time");
    console.log({ assets });

    const [thickness, setThickness] = useState(Object.fromEntries(assets.map(asset => [asset, 1])));
    const [stacked, setStacked] = useState(true);

    const rainbow = new Rainbow();
    rainbow.setNumberRange(0, assets.length + 1);

    const handleMouseEnter = (o) => {
        const { dataKey } = o;
        setThickness({ ...thickness, [dataKey]: 3 });
    };

    const handleMouseLeave = (o) => {
        const { dataKey } = o;
        setThickness({ ...thickness, [dataKey]: 1 });
    };

    const handleDateClicked = (payload) => {
        const { activeTooltipIndex } = payload;
        onDateClicked(activeTooltipIndex);
    }

    return (
        <>
            <Checkbox label="Show stacked" isSelected={stacked} onCheckboxChange={e => setStacked(e.target.checked)} />

            <ResponsiveContainer width="60%" height={500}>
                <AreaChart data={data} onClick={handleDateClicked}>
                    <XAxis dataKey="time" />
                    <YAxis />
                    <CartesianGrid strokeDasharray="3 3" opacity={0.2} />
                    <Tooltip />
                    <Legend onMouseEnter={handleMouseEnter} onMouseLeave={handleMouseLeave} />
                    {assets.map((name, index) => {
                        const color = `#${rainbow.colorAt(index)}`;
                        return <Area key={index} type="monotone" dataKey={name} stackId={stacked && "1"} fillOpacity={0.2} strokeWidth={thickness[name]} stroke={color} fill={color} />
                    })}
                </AreaChart>
            </ResponsiveContainer>
        </>
    )
}