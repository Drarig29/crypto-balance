import React from "react";

const Checkbox = ({ label, isSelected, onCheckboxChange }) => (
    <div style={{ color: 'white', marginBottom: 10 }}>
        <label>
            <input
                type="checkbox"
                name={label}
                checked={isSelected}
                onChange={onCheckboxChange}
            />
            {label}
        </label>
    </div>
);

export default Checkbox;