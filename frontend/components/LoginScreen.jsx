import React, { useContext, useRef, useState } from 'react';
import { Context } from '..';
import { sendRequest } from '../helpers';

const Form = ({ error, onSubmit }) => {
    const passwordRef = useRef();

    const handleSubmit = e => {
        e.preventDefault();
        onSubmit(passwordRef.current.value);
    };

    return (
        <form onSubmit={handleSubmit}>
            <h1>Login</h1>
            <div className='password'>
                <img src='assets/password.svg'></img>
                <input type='password' autoComplete='password' ref={passwordRef} placeholder='Password' />
            </div>
            {error && <p className='danger'>{error}</p>}
            <button type='submit'>Submit</button>
        </form>
    );
};

export const LoginScreen = ({ children }) => {
    const [context, setContext] = useContext(Context);
    const [message, setMessage] = useState(null);

    if (context.password)
        return children;

    const handlePassword = async (password) => {
        const response = await sendRequest('/auth', { password });

        if (response.status === 200) {
            setMessage(null);
            setContext({ ...context, password });
        } else {
            setMessage(response.body);
        }
    }

    return <Form error={message} onSubmit={handlePassword}></Form>
}