import React, { useState, useEffect } from 'react';

const SinglePixelOperations = ({
  onSinglePixelOperationsChange,
}) => {
  const [operations, setOperations] = useState({
    inverse: false,
    threshold: false,
    thresholdValue: 0,
    linearMapping: false,
    linearA: 1,
    linearB: 0,
    powerLaw: false,
    gamma: 1,
    equalize: false,
  });

  const handleCheckboxChange = (event) => {
    const { name, checked } = event.target;
    setOperations({ ...operations, [name]: checked });
  };

  const handleInputChange = (event) => {
    const { name, value } = event.target;
    setOperations({ ...operations, [name]: parseFloat(value) });
  };

  useEffect(() => {
    if (onSinglePixelOperationsChange) {
      onSinglePixelOperationsChange(operations);
    }
  }, [operations, onSinglePixelOperationsChange]);

  return (
    <div className="single-pixel-operations">
      <h4>Single Pixel Operations</h4>
      <h5>Linear Mappings</h5>
      <div>
        <label>
          <input
            type="checkbox"
            name="inverse"
            checked={operations.inverse}
            onChange={handleCheckboxChange}
          />
          Inverse
        </label>
      </div>
      <div>
        <label>
          <input
            type="checkbox"
            name="threshold"
            checked={operations.threshold}
            onChange={handleCheckboxChange}
          />
          Threshold at u =
        </label>
        <input
          type="number"
          name="thresholdValue"
          value={operations.thresholdValue}
          onChange={handleInputChange}
          disabled={!operations.threshold}
        />
      </div>
      <div>
        <label>
          <input
            type="checkbox"
            name="linearMapping"
            checked={operations.linearMapping}
            onChange={handleCheckboxChange}
          />
          Linear mapping with a =
        </label>
        <input
          type="number"
          name="linearA"
          value={operations.linearA}
          onChange={handleInputChange}
          disabled={!operations.linearMapping}
        />
        and b =
        <input
          type="number"
          name="linearB"
          value={operations.linearB}
          onChange={handleInputChange}
          disabled={!operations.linearMapping}
        />
      </div>
      <h5>Power Law Mapping</h5>
      <div>
        <label>
          <input
            type="checkbox"
            name="powerLaw"
            checked={operations.powerLaw}
            onChange={handleCheckboxChange}
          />
          Gamma =
        </label>
        <input
          type="number"
          name="gamma"
          value={operations.gamma}
          onChange={handleInputChange}
          disabled={!operations.powerLaw}
        />
      </div>
      <h5>Histogram Equalization</h5>
      <div>
        <label>
          <input
            type="checkbox"
            name="equalize"
            checked={operations.equalize}
            onChange={handleCheckboxChange}
          />
          Equalize
        </label>
      </div>
    </div>
  );
};

export default SinglePixelOperations;

