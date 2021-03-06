import React, { useContext } from "react";

import { Context } from '..';
import { toCurrency, toPercentage } from "../helpers";

export const CustomTooltip = ({ active, payload, label }) => {
    const [context] = useContext(Context);

    if (active && payload && payload.length) {
        return (
            <div className="custom-tooltip">
                {label && <p className="recharts-tooltip-label">{label}</p>}

                <ul className="recharts-tooltip-item-list">
                    {payload.map((item, index) => (
                        <li key={index} className="recharts-tooltip-item" style={{ color: item.stroke || item.payload.stroke }}>
                            <span>{`${item.name} : ${toCurrency(item.value, context)}`}</span>
                            {item.payload.percent && <span> ({toPercentage(item.payload.percent)})</span>}
                        </li>
                    ))}
                </ul>
            </div>
        );
    }

    return null;
};