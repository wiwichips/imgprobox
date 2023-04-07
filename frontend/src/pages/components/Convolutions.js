import React, { useState, useEffect } from 'react';

const Convolutions = ({
  onConvolutionsChange,
  setCustomConvolution,
}) => {
  const [width, setWidth] = useState(3);
  const [height, setHeight] = useState(3);
  const [kernel, setKernel] = useState(Array(height).fill(Array(width).fill(0)));
  const [normalize, setNormalize] = useState(false);
  const [commonKernels, setCommonKernels] = useState({
    gaussian: false,
    sobel: false,
  });
//  const [customConvolution, setCustomConvolution] = useState(null);

  const updateKernelValue = (row, col, value) => {
    const newKernel = [...kernel];
    newKernel[row][col] = parseFloat(value);
    setKernel(newKernel);
  };

  const handleKernelSizeChange = () => {
    const newKernel = Array(height).fill(Array(width).fill(0));
    setKernel(newKernel);
  };

    const applyCustomConvolution = () => {
      const newConvolution = {
        kernel: kernel,
        normalize: normalize,
      };
      setCustomConvolution(newConvolution);
    };

  useEffect(() => {
    if (onConvolutionsChange) {
      onConvolutionsChange({ kernel, normalize, commonKernels });
    }
    console.log("useEffect");
  }, [kernel, normalize, commonKernels]);

    useEffect(() => {
      const newKernel = Array(height).fill(Array(width).fill(0)).map((row) => row.slice());
      setKernel(newKernel);
    }, [width, height]);

  return (
    <div className="convolutions">
      <h4>Convolutions</h4>
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
                    onChange={(e) =>
                      updateKernelValue(rowIndex, colIndex, e.target.value)
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
            type="checkbox"
            checked={commonKernels.gaussian}
            onChange={(e) =>
              setCommonKernels({ ...commonKernels, gaussian: e.target.checked })
            }
          />
          Gaussian
        </label>
      </div>
      <div>
        <label>
          <input
            type="checkbox"
            checked={commonKernels.sobel}
            onChange={(e) =>
              setCommonKernels({ ...commonKernels, sobel: e.target.checked })
            }
          />
          Sobel
        </label>
      </div>
      {/* Add more common kernels as needed */}
    </div>
  );
};

export default Convolutions;

