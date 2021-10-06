import React, { useState } from 'react';

export const VisibilityButton = ({ initiallyRevealed, onRevealedChange }) => {
    const [revealed, setRevealed] = useState(initiallyRevealed);
    const src = revealed ? 'assets/visibility_off.svg' : 'assets/visibility.svg';

    const handleClick = () => {
        const toggled = !revealed;
        setRevealed(toggled);
        onRevealedChange(toggled);
    }

    return (
        <a className='reveal-btn' onClick={handleClick}>
            <img src={src}></img>
        </a>
    )
}