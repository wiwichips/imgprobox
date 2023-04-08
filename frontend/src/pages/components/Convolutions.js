import React, { useState, useEffect } from 'react';

const prebuiltKernels = {
  identity: {
    kernel: [
      [1],
    ],
    size: 1,
    normalize: false,
  },
  gaussian: {
    kernel: [
      [1, 2, 1],
      [2, 4, 2],
      [1, 2, 1],
    ],
    size: 3,
    normalize: true,
  },
  gaussian_5x5: {
    kernel: [
      [1, 4, 6, 4, 1],
      [4, 16, 24, 16, 4],
      [6, 24, 36, 24, 6],
      [4, 16, 24, 16, 4],
      [1, 4, 6, 4, 1],
    ],
    size: 5,
    normalize: true,
  },
  laplacian: {
    kernel: [
      [1, 1, 1],
      [1, -8, 1],
      [1, 1, 1],
    ],
    size: 3,
    normalize: false,
  },
  laplacian_of_gaussian: {
    kernel: [
      [0, 0, -1, 0, 0],
      [0, -1, -2, -1, 0],
      [-1, -2, 16, -2, -1],
      [0, -1, -2, -1, 0],
      [0, 0, -1, 0, 0],
    ],
    size: 5,
    normalize: false,
  }
};

const Convolutions = ({
  onConvolutionsChange,
  setCustomConvolution,
}) => {
  const [width, setWidth] = useState(3);
  const [height, setHeight] = useState(3);
  const [manualWidth, setManualWidth] = useState(3);
  const [manualHeight, setManualHeight] = useState(3);

  const [kernel, setKernel] = useState(
    Array.from({ length: height }, () => Array.from({ length: width }, () => 0))
  );
  const [enabled, setEnabled] = useState(false);
  const [normalize, setNormalize] = useState(true);
  const [checkedKernel, setCheckedKernel] = useState("custom");

  const updateKernelValue = (row, col, value) => {
    const newKernel = [...kernel];
    newKernel[row][col] = parseFloat(value);
    setKernel(newKernel);
  };

  const applyCustomConvolution = () => {
    const newConvolution = {
      kernel: kernel,
      normalize: normalize,
    };
    onConvolutionsChange(newConvolution);
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
  }, [manualWidth, manualHeight]);

  const pbj = function prebuiltKernelJsx () {
    const kernelJsx = [];
    for (const [name, kernel] of Object.entries(prebuiltKernels)) {
      kernelJsx.push (
        <div key={name}>
          <label>
            <input
              type="radio"
              value={name}
              checked={checkedKernel === name}
              onChange={function (e) {
                //handleCommonKernelChange(e);
                if (kernel.normalize != normalize) {
                  setNormalize(kernel.normalize);
                }
                if (kernel.width != width) {
                  setWidth(kernel.size);
                }
                if (kernel.height != height) {
                  setHeight(kernel.size);
                }
                setKernel(kernel.kernel);
                setCheckedKernel(name);
              }}
            />
            {name}
          </label>
        </div>
      )
    }
    return kernelJsx;
  }

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
        <button onClick={applyCustomConvolution}>Set Kernel</button>
      </div>
      <h4>Kernel Display</h4>
      <div>
        <label>
          Width:
          <input className="show-arrows" type="number" min="1" max="10" value={width}
            onChange={(e) => {
              const newValue = parseInt(e.target.value) || 1;
              const newWidth = Math.min(Math.max(newValue, 1), 10);
              setWidth(newWidth);
              setManualWidth(newWidth);
            }}
          />

        </label>
        <label>
          Height:
          <input className="show-arrows" type="number" min="1" max="10" value={height}
            onChange={(e) => {
              const newValue = parseInt(e.target.value) || 1;
              const newHeight = Math.min(Math.max(newValue, 1), 10);
              setHeight(newHeight);
              setManualHeight(newHeight);
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
      <h5>Pre-built Kernels</h5>
      {pbj()}
    </div>
  );
};

export default Convolutions;

