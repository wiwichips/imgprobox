import React, { useState, useEffect } from 'react';

const GeometricTransformations = ({
  onTransformationsChange,
}) => {
  const [transformations, setTransformations] = useState({
    mirror: false,
    flip: false,
    rotate: 0,
    scale: 100,
    scalingMethod: 'nearest',
    crop: false,
  });

  const handleCheckboxChange = (event) => {
    const { name, checked } = event.target;
    setTransformations({ ...transformations, [name]: checked });
  };

  const handleSliderChange = (event) => {
    const { name, value } = event.target;
    setTransformations({ ...transformations, [name]: parseFloat(value) });
  };

  const handleRadioChange = (event) => {
    const { name, value } = event.target;
    setTransformations({ ...transformations, [name]: value });
  };

  useEffect(() => {
    if (onTransformationsChange) {
      onTransformationsChange(transformations);
    }
  }, [transformations, onTransformationsChange]);

  return (
    <div className="geometric-transformations">
      <h4>Geometric Spatial Transformations</h4>
      <div>
        <label>
          <input
            type="checkbox"
            name="crop"
            checked={transformations.crop}
            onChange={handleCheckboxChange}
          />
         Crop Image 
        </label>
      </div>
      <div>
        <label>
          <input
            type="checkbox"
            name="mirror"
            checked={transformations.mirror}
            onChange={handleCheckboxChange}
          />
          Mirror Image
        </label>
      </div>
      <div>
        <label>
          <input
            type="checkbox"
            name="flip"
            checked={transformations.flip}
            onChange={handleCheckboxChange}
          />
          Flip upside down
        </label>
      </div>
      <div>
        <label>
          Rotate
          <input
            type="range"
            name="rotate"
            min="0"
            max="360"
            value={transformations.rotate}
            onChange={handleSliderChange}
          />
        </label>
      </div>
      <div>
        <label>
          <input
            type="number"
            name="scale"
            min="1"
            max="500"
            value={transformations.scale}
            onChange={handleSliderChange}
          />
          % Scale Image
        </label>
      </div>
      <div>
        <label>
          <input
            type="radio"
            name="scalingMethod"
            value="nearest"
            checked={transformations.scalingMethod === 'nearest'}
            onChange={handleRadioChange}
          />
          Nearest neighbour
        </label>
      </div>
      <div>
        <label>
          <input
            type="radio"
            name="scalingMethod"
            value="bilinear"
            checked={transformations.scalingMethod === 'bilinear'}
            onChange={handleRadioChange}
          />
          Bi-linear interpolation
        </label>
      </div>
    </div>
  );
};

export default GeometricTransformations;

