import React, { useRef, useState } from 'react';
import moment from 'moment';

import DayPickerInput from 'react-day-picker/DayPickerInput';
import 'react-day-picker/lib/style.css';

import { formatDate, parseDate } from 'react-day-picker/moment';

export const DatePicker = ({ initialRange, onRangeChange }) => {
    const [from, setFrom] = useState(initialRange.from);
    const [to, setTo] = useState(initialRange.to);
    const toPickerRef = useRef();

    const showFromMonth = () => {
        if (!from) return;
        if (moment(to).diff(moment(from), 'months') < 2) {
            toPickerRef.current.getDayPicker().showMonth(from);
        }
    }

    const handleFromChange = (from) => {
        setFrom(from);
    }

    const handleToChange = (to) => {
        setTo(to);
        showFromMonth();
        onRangeChange(from, to);
    }

    const modifiers = { start: from, end: to };

    return (
        <div className="InputFromTo" style={{ margin: 20 }}>
            <span style={{ color: 'white' }}>Filter data from </span>
            <DayPickerInput
                value={from}
                placeholder="From"
                format="LL"
                formatDate={formatDate}
                parseDate={parseDate}
                dayPickerProps={{
                    selectedDays: [from, { from, to }],
                    disabledDays: { after: to },
                    toMonth: to,
                    modifiers,
                    numberOfMonths: 2,
                    onDayClick: () => toPickerRef.current.getInput().focus(),
                }}
                onDayChange={handleFromChange}
            />
            <span style={{ color: 'white' }}> to </span>
            <span className="InputFromTo-to">
                <DayPickerInput
                    ref={toPickerRef}
                    value={to}
                    placeholder="To"
                    format="LL"
                    formatDate={formatDate}
                    parseDate={parseDate}
                    dayPickerProps={{
                        selectedDays: [from, { from, to }],
                        disabledDays: { before: from, after: new Date() },
                        modifiers,
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