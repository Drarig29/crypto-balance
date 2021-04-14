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
        <a onClick={handleClick}>
            <img className="reveal-button" src={src}></img>
        </a>
    )
}