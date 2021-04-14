import React from 'react';
import ReactDOM from 'react-dom';

import AreaChart from './AreaChart';

import './style.css';

const App = () => (
    <>
        <AreaChart />
    </>
)

ReactDOM.render(<App />, document.getElementById('app'));
