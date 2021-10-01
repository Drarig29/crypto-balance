import React, { useRef, useState } from 'react';
import moment from 'moment';

import DayPickerInput from 'react-day-picker/DayPickerInput';
import 'react-day-picker/lib/style.css';

import { formatDate, parseDate } from 'react-day-picker/moment';

export const DatePicker = ({ initialRange, onRangeChange }) => {
    const [from, setFrom] = useState(initialRange.from);
    const [to, setTo] = useState(initialRange.to);
    const toPickerRef = useRef();

    const updateFromMonth = () => {
        if (!from) return;
        if (moment(to).diff(moment(from), 'months') < 2) {
            toPickerRef.current.getDayPicker().showMonth(from);
        }
    }

    const handleFromChange = (from) => {
        setFrom(from);
        onRangeChange(from, to);
    }

    const handleToChange = (to) => {
        setTo(to);
        updateFromMonth();
        onRangeChange(from, to);
    }

    const props = {
        inputProps: { readOnly: true },
        format: "LL",
        formatDate,
        parseDate,
    }

    const modifiers = { start: from, end: to };

    return (
        <div className="InputFromTo" style={{ margin: 20 }}>
            <span style={{ color: 'white' }}>Filter data from </span>
            <DayPickerInput
                {...props}
                value={from}
                placeholder="From"
                inputProps={{ readOnly: true }}
                dayPickerProps={{
                    modifiers,
                    selectedDays: [from, { from, to }],
                    disabledDays: { after: to },
                    toMonth: to,
                    numberOfMonths: 2,
                }}
                onDayChange={handleFromChange}
            />
            <span style={{ color: 'white' }}> to </span>
            <span className="InputFromTo-to">
                <DayPickerInput
                    {...props}
                    ref={toPickerRef}
                    value={to}
                    placeholder="To"
                    dayPickerProps={{
                        modifiers,
                        selectedDays: [from, { from, to }],
                        disabledDays: { before: from, after: new Date() },
                        month: from,
                        fromMonth: from,
                        numberOfMonths: 2,
                    }}
                    onDayChange={handleToChange}
                />
            </span>
        </div>
    );
}