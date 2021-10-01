import React, { useContext, useMemo, useState } from 'react';

import Rainbow from 'rainbowvis.js';
import { YAxis, Tooltip, Area, CartesianGrid, XAxis, AreaChart as Chart, Legend, ResponsiveContainer } from 'recharts';

import { Checkbox } from './CheckBox';
import { toCurrency } from '../helpers';
import { Context } from "..";
import { CustomTooltip } from './CustomTooltip';

export const AreaChart = ({ data, onDateClicked }) => {
    const [context] = useContext(Context);

    const assets = useMemo(() => (data.length > 0 && Object.keys(data[0]) || []).filter(key => key !== "Total as BTC" && key !== "time"), [data]);

    const [thickness, setThickness] = useState(Object.fromEntries([...assets, 'Total as BTC'].map(asset => [asset, 1])));
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

            <ResponsiveContainer width="90%" height={500}>
                <Chart data={data} onClick={handleDateClicked}>
                    <XAxis dataKey="time" />
                    <YAxis tickFormatter={value => toCurrency(value, context, 0)} />
                    <CartesianGrid strokeDasharray="3 3" opacity={0.2} />
                    <Tooltip content={CustomTooltip} />
                    <Legend onMouseEnter={handleMouseEnter} onMouseLeave={handleMouseLeave} />

                    {assets.map((name, index) => {
                        const color = `#${rainbow.colorAt(index)}`;
                        return <Area key={index} type="monotone" dataKey={name} stackId={stacked && "1"} fillOpacity={0.2} strokeWidth={thickness[name]} stroke={color} fill={color} />
                    })}

                    {stacked && <Area type="monotone" dataKey="Total as BTC" strokeWidth={thickness['Total as BTC']} strokeDasharray="3 3" stroke="red" fill="transparent" />}
                </Chart>
            </ResponsiveContainer>
        </>
    )
}