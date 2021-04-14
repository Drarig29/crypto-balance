export function toISOString(date) {
    return date.toISOString().split('T')[0] + 'T00:00:00Z';
}

export function toCurrency(value, context, digits = 2) {
    if (value === 0)
        return '0';

    const amount = `${value.toFixed(digits)} ${context.currency.symbol}`;

    if (context.revealValues)
        return amount;

    return amount.replace(/\d/g, '-');
}