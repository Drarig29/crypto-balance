import React, { createContext, useEffect, useState } from 'react';
import ReactDOM from 'react-dom';

import { DataContainer } from './components/DataContainer';

import './style.css';

const defaultValues = {
    revealValues: true,
    currency: {
        name: 'EUR',
        symbol: '€',
    },
};

const Context = createContext();

const App = () => {
    const settings = JSON.parse(window.localStorage.getItem('settings')) || defaultValues;
    const [state, setState] = useState(settings);

    useEffect(() => {
        window.localStorage.setItem('settings', JSON.stringify(state));
    }, [state]);

    return (
        <main>
            <Context.Provider value={[state, setState]}>
                <DataContainer />
            </Context.Provider>
        </main>
    )
}

ReactDOM.render(<App />, document.getElementById('app'));

export { Context };