import React, { useContext, useRef } from 'react';
import { Context } from '..';

const Form = ({ onSubmit }) => {
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
            <button type='submit'>Submit</button>
        </form>
    );
};

export const LoginScreen = ({ children }) => {
    const [context, setContext] = useContext(Context);

    if (context.password)
        return children

    return <Form onSubmit={password => setContext({ ...context, password })}></Form>
}