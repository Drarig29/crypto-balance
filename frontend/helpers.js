export function toISOString(date) {
    return date.toISOString().split('T')[0] + 'T00:00:00Z';
}

export function toCurrency(value, currency, digits = 2) {
    if (value === 0)
        return '0';

    return `${value.toFixed(digits)} ${currency}`;
}