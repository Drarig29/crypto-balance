import React, { createContext, useEffect, useState } from 'react';
import ReactDOM from 'react-dom';

import { DataContainer } from './components/DataContainer';
import { LoginScreen } from './components/LoginScreen';

import './style.css';

const defaultValues = {
    password: null,
    revealValues: true,
    showAssetAmount: false,
    currency: {
        name: 'EUR',
        symbol: '€',
    },
};

const Context = createContext();

function load() {
    const raw = window.localStorage.getItem('settings');
    if (!raw) return null;
    const decoded = decodeURIComponent(atob(raw));
    return JSON.parse(decoded);
}

function save(state) {
    const raw = JSON.stringify(state);
    const encoded = btoa(encodeURIComponent(raw));
    window.localStorage.setItem('settings', encoded);
}

const App = () => {
    const settings = load() || defaultValues;
    const [state, setState] = useState(settings);

    useEffect(() => {
        save(state)
    }, [state]);

    return (
        <Context.Provider value={[state, setState]}>
            <LoginScreen>
                <DataContainer />
            </LoginScreen>
        </Context.Provider>
    )
}

ReactDOM.render(<App />, document.getElementById('app'));

export { Context };