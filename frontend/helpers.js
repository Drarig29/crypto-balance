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

export function toAssetAmount(value, context) {
    const amount = value.toFixed(4);

    if (context.revealValues)
        return amount;

    return amount.replace(/\d/g, '-');
}

export function toPercentage(percent) {
    return `${(percent * 100).toFixed(2)} %`;
}

export function toDateString(date) {
    if (typeof date.$date === 'string') {
        return new Date(date.$date).toDateString()
    }

    return new Date(parseInt(date.$date.$numberLong)).toDateString()
}

export async function sendRequest(url, body) {
    body = JSON.stringify(body);

    const headers = new Headers();
    headers.append("Content-Type", "application/json");

    const options = {
        method: 'POST',
        headers,
        body,
    };

    const response = await fetch(url, options);
    const obj = await response.json();

    return {
        status: response.status,
        body: obj,
    };
}