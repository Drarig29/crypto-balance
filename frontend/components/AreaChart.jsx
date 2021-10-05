import React, { useContext, useMemo, useState } from 'react';

import Rainbow from 'rainbowvis.js';
import { YAxis, Tooltip, Area, CartesianGrid, XAxis, AreaChart as Chart, Legend, ResponsiveContainer } from 'recharts';

import { Checkbox } from './CheckBox';
import { toCurrency } from '../helpers';
import { Context } from "..";
import { CustomTooltip } from './CustomTooltip';

const DEFAULT_THICKNESS = 1;
const HIGHLIGHT_THICKNESS = 3;

export const AreaChart = ({ data, onDateClicked }) => {
    const [context] = useContext(Context);

    const assets = useMemo(() => (data.length > 0 && Object.keys(data[0]) || []).filter(key => key !== "Total as BTC" && key !== "time"), [data]);
    const allAssets = [...assets, 'Total as BTC']

    const [thickness, setThickness] = useState(Object.fromEntries(allAssets.map(asset => [asset, DEFAULT_THICKNESS])));
    const [selectedAsset, setSelectedAsset] = useState(null);
    const [stacked, setStacked] = useState(true);
    const [showAssetAmount, setShowAssetAmount] = useState(false);

    const values = useMemo(() => (
        data.map(d => allAssets.reduce((acc, asset) => ({ ...acc, [asset]: d[asset]?.[showAssetAmount ? 'amount' : 'value'] }), d))
    ), [
        assets,
        showAssetAmount
    ]);

    const rainbow = new Rainbow();
    rainbow.setNumberRange(0, assets.length + 1);

    const handleMouseEnter = (props) => {
        const { dataKey } = props;
        setThickness({ ...thickness, [dataKey]: HIGHLIGHT_THICKNESS });
    };

    const handleMouseLeave = (props) => {
        const { dataKey } = props;
        setThickness({ ...thickness, [dataKey]: DEFAULT_THICKNESS });
    };

    const handleMouseClick = (props) => {
        const { dataKey, color } = props;

        if (selectedAsset) {
            setSelectedAsset(null);
            setShowAssetAmount(false);
        } else {
            setSelectedAsset({
                name: dataKey,
                color,
            });
        }
    };

    const handleDateChanged = (payload) => {
        if (!payload) return;
        const { activeTooltipIndex } = payload;
        onDateClicked(activeTooltipIndex);
    };

    const valueFormatter = showAssetAmount ? (value => value.toFixed(4)) : toCurrency;

    return (
        <>
            <Checkbox label="Show stacked" isSelected={stacked} onCheckboxChange={e => setStacked(e.target.checked)} />

            {selectedAsset && (
                <Checkbox label="Show asset value" isSelected={showAssetAmount} onCheckboxChange={e => setShowAssetAmount(e.target.checked)} />
            )}

            <ResponsiveContainer width="90%" height={500}>
                <Chart data={values} onClick={handleDateChanged}>
                    <XAxis dataKey="time" />
                    <YAxis tickFormatter={value => valueFormatter(value, context, 0)} />

                    <CartesianGrid strokeDasharray="3 3" opacity={0.2} />
                    <Tooltip content={CustomTooltip} valueFormatter={valueFormatter} />
                    <Legend onMouseEnter={handleMouseEnter} onMouseLeave={handleMouseLeave} onMouseUp={handleMouseClick} />

                    {selectedAsset ? (
                        <Area type="monotone" dataKey={selectedAsset.name} strokeWidth={HIGHLIGHT_THICKNESS} stroke={selectedAsset.color} fill={selectedAsset.color} />
                    ) : (
                        assets.map((name, index) => {
                            const color = `#${rainbow.colorAt(index)}`;
                            return <Area key={index} type="monotone" dataKey={name} stackId={stacked && "1"} fillOpacity={0.2} strokeWidth={thickness[name]} stroke={color} fill={color} />
                        })
                    )}

                    {stacked && !selectedAsset && (
                        <Area type="monotone" dataKey="Total as BTC" strokeWidth={thickness['Total as BTC']} strokeDasharray="3 3" stroke="red" fill="transparent" />
                    )}
                </Chart>
            </ResponsiveContainer>
        </>
    )
}