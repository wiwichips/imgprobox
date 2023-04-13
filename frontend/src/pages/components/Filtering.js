import React, { useEffect, useState } from 'react';

const Filtering = ({ onFilteringChange }) => {
  const [salt, setSalt] = useState(0);
  const [pepper, setPepper] = useState(0);
  const [filterType, setFilterType] = useState('none');
  const [neighborhoodType, setNeighborhoodType] = useState('chessboard');
  const [neighborhoodSize, setNeighborhoodSize] = useState(1);

  useEffect(() => {
    if (onFilteringChange) {
      onFilteringChange({
        salt,
        pepper,
        filterType,
        neighborhoodType,
        neighborhoodSize,
      });
    }
  }, [salt, pepper, filterType, neighborhoodType, neighborhoodSize]);

  return (
    <div className="filtering">
      <h4>Filtering</h4>

      <div>
        <h5>Add Noise</h5>
        <div>
            <label>
            <input
                type="number"
                value={salt}
                min="0"
                max="100"
                onChange={(e) => {
                    setSalt(Math.max(0, Math.min(100, parseFloat(e.target.value))));
                }}
            />
            % Salt
            </label>
        </div>
        <div>
            <label>
            <input
                type="number"
                value={pepper}
                min="0"
                max="100"
                onChange={(e) => {
                    setPepper(Math.max(0, Math.min(100, parseFloat(e.target.value))));
                }}
            />
            % Pepper
            </label>
        </div>
      </div>

      <div>
        <h5>Filtering</h5>
        <div>
            <label>
            <input
                type="radio"
                value="max"
                checked={filterType === 'none'}
                onChange={() => {
                    setFilterType('none');
                }}
            />
            None
            </label>
        </div>
        <div>
            <label>
            <input
                type="radio"
                value="max"
                checked={filterType === 'max'}
                onChange={() => {
                    setFilterType('max');
                }}
            />
            Max
            </label>
        </div>
        <div>
            <label>
            <input
                type="radio"
                value="median"
                checked={filterType === 'median'}
                onChange={() => {
                    setFilterType('median');
                }}
            />
            Median
            </label>
        </div>
        <div>
            <label>
            <input
                type="radio"
                value="min"
                checked={filterType === 'min'}
                onChange={() => {
                    setFilterType('min');
                }}
            />
            Min
            </label>
        </div>
      </div>

      <div>
        <h5>Neighbourhood Type</h5>
        <div>
            <label>
            Neighbourhood size:
            <input
                type="number"
                value={neighborhoodSize}
                min="1"
                onChange={(e) => {
                    setNeighborhoodSize(Math.max(1, parseInt(e.target.value)));
                }}
            />
            </label>
        </div>
        <div>
            <label>
            <input
                type="radio"
                value="chessboard"
                checked={neighborhoodType === 'chessboard'}
                onChange={() => {
                    setNeighborhoodType('chessboard');
                }}
            />
            d8 (Chessboard)
            </label>
        </div>
        <div>
            <label>
            <input
                type="radio"
                value="cityBlock"
                checked={neighborhoodType === 'cityBlock'}
                onChange={() => {
                    setNeighborhoodType('cityBlock');
                }}
            />
            d4 (City block)
            </label>
        </div>
      </div>
    </div>
  );
};

export default Filtering;
