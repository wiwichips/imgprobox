import React, { useState, useEffect } from 'react';

const Padding = ({ onPaddingTypeChange }) => {
  const [paddingType, setPaddingType] = useState('reflected');

  const handleChange = (event) => {
    setPaddingType(event.target.value);
  };

  useEffect(() => {
    if (onPaddingTypeChange) {
      onPaddingTypeChange(paddingType);
    }
  }, [paddingType, onPaddingTypeChange]);

  return (
    <div className="padding">
      <h4>Padding</h4>
      <div className="padding-container">
        <label className="radio-label">
          <input type="radio" value="reflected"
            checked={paddingType === 'reflected'}
            onChange={handleChange}
          />
          Reflected indexing
        </label>
        <label className="radio-label">
          <input type="radio" value="circular"
            checked={paddingType === 'circular'}
            onChange={handleChange}
          />
          Circular indexing
        </label>
        <label className="radio-label">
          <input type="radio" value="zero"
            checked={paddingType === 'zero'}
            onChange={handleChange}
          />
          Zero padding
        </label>
      </div>
    </div>
  );
};

export default Padding;

