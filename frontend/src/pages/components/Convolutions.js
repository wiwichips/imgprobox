import React, { useState, useEffect } from 'react';

const Convolutions = ({
  onConvolutionsChange,
  setCustomConvolution,
}) => {
  const [width, setWidth] = useState(3);
  const [height, setHeight] = useState(3);
  const [kernel, setKernel] = useState(
    Array.from({ length: height }, () => Array.from({ length: width }, () => 0))
  );
  const [enabled, setEnabled] = useState(false);
  const [normalize, setNormalize] = useState(true);
  const [checkedKernel, setCheckedKernel] = useState("custom");
  const [applyConvolution, setApplyConvolution] = useState(false);

  const updateKernelValue = (row, col, value) => {
    const newKernel = [...kernel];
    newKernel[row][col] = parseFloat(value);
    setKernel(newKernel);
  };

  // is this deprectaed?
  const handleKernelSizeChange = () => {
    const newKernel = Array(height).fill(Array(width).fill(0));
    setKernel(newKernel);
  };

  const handleCommonKernelChange = (event) => {
    const kernelType = event.target.value;
    setCheckedKernel(kernelType);

    if (kernelType == "gaussian") {
      setKernel([
        [1, 2, 1],
        [2, 4, 2],
        [1, 2, 1],
      ]);
      setNormalize(true);
    }
    
    else if (kernelType == "sobel") {
      setKernel([
        [1, 0, -1],
        [2, 0, -2],
        [1, 0, -1],
      ]);
      setNormalize(false);
    } 

    else if (kernelType == "custom") {
      setKernel(
        Array.from({ length: height }, () => Array.from({ length: width }, () => 0))
      );
      setNormalize(false);
    }
    
    else {
      console.log("ERROR: invalid checkedKernel: ", checkedKernel);
    }
  };

  const applyCustomConvolution = () => {
    const newConvolution = {
      kernel: kernel,
      normalize: normalize,
    };
    onConvolutionsChange(newConvolution);
  };

  const handleApplyCustomConvolutionChange = (event) => {
    setApplyConvolution(event.target.checked);
    if (event.target.checked) {
      applyCustomConvolution();
    }
  };

  const applyEnabledConvolutions = (event) => {
    setCustomConvolution(event.target.checked);
    setEnabled(event.target.checked);
  };
    


  useState(() => {
    console.log('!!! + end of useEffect for onConvolutionsChange:', kernel[0])
  }, [kernel, normalize]);

  useEffect(() => {
    const newKernel = Array(height).fill(Array(width).fill(0)).map((row) => row.slice());
    setKernel(newKernel);
  }, [width, height]);

  return (
    <div className="convolutions">
      <h4>Convolutions</h4>
      <div>
        <label>
          <input
            type="checkbox"
            checked={enabled}
            onChange={applyEnabledConvolutions}
          />
          Enable
        </label>
      </div>

      <div>
        <button onClick={applyCustomConvolution}>Apply Custom Convolution</button>
        <label>
          Width:
          <input className="show-arrows" type="number" min="1" max="10" value={width}
            onChange={(e) => {
              const newValue = parseInt(e.target.value) || 1;
              setWidth(Math.min(Math.max(newValue, 1), 10));
            }}
          />

        </label>
        <label>
          Height:
          <input className="show-arrows" type="number" min="1" max="10" value={height}
            onChange={(e) => {
              const newValue = parseInt(e.target.value) || 1;
              setHeight(Math.min(Math.max(newValue, 1), 10));
            }}
          />
        </label>
      </div>
      <table>
        <tbody>
          {kernel.map((row, rowIndex) => (
            <tr key={rowIndex}>
              {row.map((value, colIndex) => (
                <td key={colIndex}>
                  <input
                    type="number"
                    value={value}
                    pattern="\d*"
                    inputMode="numeric"
                    onChange={(e) => {
                        if (e.target.value == "" || !isNaN(e.target.value)) {
                          updateKernelValue(rowIndex, colIndex, e.target.value);
                        } else {
                          updateKernelValue(rowIndex, colIndex, 0);
                        }
                      }
                    }
                  />
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
      <div>
        <label>
          <input
            type="checkbox"
            checked={normalize}
            onChange={(e) => setNormalize(e.target.checked)}
          />
          Normalize
        </label>
      </div>
      <h5>Common Kernels</h5>
      <div>
        <label>
          <input
            type="radio" value="custom"
            checked={checkedKernel == "custom"}
            onChange={handleCommonKernelChange}
          />
          Custom
        </label>
      </div>
      <div>
        <label>
          <input
            type="radio" value="gaussian"
            checked={checkedKernel == "gaussian"}
            onChange={handleCommonKernelChange}
          />
          Gaussian
        </label>
      </div>
      <div>
        <label>
          <input
            type="radio" value="sobel"
            checked={checkedKernel == "sobel"}
            onChange={handleCommonKernelChange}
          />
          Sobel
        </label>
      </div>
      {/* Add more common kernels as needed */}
    </div>
  );
};

export default Convolutions;

