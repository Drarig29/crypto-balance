import React from 'react';
import ReactDOM from 'react-dom';

import Rainbow from 'rainbowvis.js';

import { YAxis } from 'recharts';
import { Tooltip } from 'recharts';
import { Area } from 'recharts';
import { CartesianGrid } from 'recharts';
import { XAxis } from 'recharts';
import { AreaChart } from 'recharts';

export default function ({ data }) {
    const assets = (data.length > 0 && Object.keys(data[0]) || []).filter(key => key !== "Total as BTC" && key !== "time");
    console.log(assets);

    const rainbow = new Rainbow();
    rainbow.setNumberRange(0, assets.length + 1);

    return (
        <AreaChart width={1000} height={500} data={data}
            margin={{ top: 10, right: 30, left: 0, bottom: 0 }}>
            <XAxis dataKey="time" />
            <YAxis />
            <CartesianGrid strokeDasharray="3 3" />
            <Tooltip />
            {assets.map((name, index) => {
                const color = `#${rainbow.colorAt(index)}`;
                return <Area key={index} type="monotone" dataKey={name} stackId="1" stroke={color} fill={color} />
            })}
        </AreaChart>
    )
}