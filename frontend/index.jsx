import React from 'react';
import ReactDOM from 'react-dom';

import DataContainer from './DataContainer';

import './style.css';

const App = () => (
    <main>
        <DataContainer />
    </main>
)

ReactDOM.render(<App />, document.getElementById('app'));
