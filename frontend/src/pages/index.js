import React, { useEffect, useRef, useState, useCallback } from 'react';
import 'bootstrap/dist/css/bootstrap.min.css';
import init, { draw } from '../../../pkg/without_a_bundler'; // Replace with the correct import
import ExpandableSection from './components/ExpandableSection';
import Padding from './components/Padding';
import GeometricTransformations from './components/GeometricTransformations';
import SinglePixelOperations from './components/SinglePixelOperations';
import Convolutions from './components/Convolutions';

function App() {
  const canvasRef = useRef();
  const videoRef = useRef();
  const fileInputRef = useRef();
  const buttonRef = useRef();
  let videoOn = false;

  const handleWasmDrawRef = useRef();

  // padding
  const [selectedPaddingType, setSelectedPaddingType] = useState('reflected');

  const handlePaddingTypeChange = (paddingType) => {
    setSelectedPaddingType(paddingType);
    console.log(paddingType);
  };

  // geometric transformations
  const [transformations, setTransformations] = useState({
    mirror: false,
    flip: false,
    rotate: 0,
    scale: 100,
    scalingMethod: 'nearest',
  });

  const handleTransformationsChange = (newTransformations) => {
    setTransformations(newTransformations);
  };

  // single pixel operations
  const [singlePixelOperations, setSinglePixelOperations] = useState({
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

  const handleSinglePixelOperationsChange = (newOperations) => {
    setSinglePixelOperations(newOperations);
  };

  // convolutions
  const [convolutionEnabled, setConvlutionEnabled] = useState(false);
  const [convolutions, setConvolutions] = useState({
    kernel: Array(3).fill(Array(3).fill(0)),
    normalize: false,
  });

  const handleConvolutionsChange = (newConvolutions) => {
    setConvolutions(newConvolutions);
  };


  const handleWasmDraw = useCallback((canvasObj, canvasWidth, canvasHeight) => {
    const options = [
      convolutionEnabled ? 'convolutionDemo' : null,
    ].filter((option) => option !== null);

    // Call the draw function from wasm
    let kernel = convolutions.kernel;
    if (convolutions.normalize) {
      const sum = kernel.reduce((acc, row) => acc + row.reduce((acc, val) => acc + val, 0), 0);
      kernel = kernel.map((row) => row.map((val) => val / sum));
    }

    const spo_array = [];
    if (singlePixelOperations.inverse) {
      spo_array.push({op_type: 'linear', a: -1, b: 255});
    }
    if (singlePixelOperations.threshold) {
      spo_array.push({op_type: 'threshold', a: singlePixelOperations.thresholdValue, b: 0});
    } if (singlePixelOperations.linearMapping) {
      spo_array.push({op_type: 'linear', a: singlePixelOperations.linearA, b: singlePixelOperations.linearB});
    } if (singlePixelOperations.powerLaw) {
      spo_array.push({op_type: 'powerLaw', a: singlePixelOperations.gamma, b: 0});
    } if (singlePixelOperations.equalize) {
      spo_array.push({op_type: 'histogram_equalization', a: 0, b: 0});
    }

    draw(canvasObj, canvasWidth, canvasHeight, options, kernel, spo_array, transformations.rotate, transformations.scale/100);
  }, [singlePixelOperations, convolutions, transformations]);

  useEffect(() => {
    handleWasmDrawRef.current = handleWasmDraw;
  }, [handleWasmDraw]);

  function handleImage(e) {
    var reader = new FileReader();
    const img = new Image();
    img.onload = () => {
      // Set canvas dimensions to match the uploaded image
      console.log("JS --> " + img.width + " and " + img.height);
      canvas.width = img.width;
      canvas.height = img.height;

      canvas.getContext('2d').drawImage(img, 0, 0);
      //draw(canvas.getContext('2d'), canvas.width, canvas.height);
      handleWasmDrawRef.current(canvas.getContext('2d'), canvas.width, canvas.height);
    };
    reader.onload = function (event) {
      img.src = event.target.result;
    };
    reader.readAsDataURL(e.target.files[0]);
  }

  useEffect(() => {
    const canvas = canvasRef.current;
    const video = videoRef.current;
    const button = buttonRef.current;
    const fileInput = fileInputRef.current;
    let localStream;

    async function initialize() {
      await init();

      button.onclick = flipVideoFeed;
      fileInput.addEventListener('change', handleImage, false);

      function processFrame() {
        canvas.width = video.videoWidth === 0 ? 640 : video.videoWidth;
        canvas.height = video.videoHeight === 0 ? 480 : video.videoHeight;

        canvas.getContext('2d').drawImage(video, 0, 0, canvas.width, canvas.height);

        let image = new Image();
        image.src = canvas.toDataURL();
        canvas.getContext('2d').drawImage(image, 0, 0);

        console.log(video.videoWidth + " and " + video.videoHeight)
        handleWasmDrawRef.current(canvas.getContext('2d'), canvas.width, canvas.height);

        if (videoOn) {
          window.requestAnimationFrame(processFrame);
        }
      }

      function flipVideoFeed() {
        if (!videoOn) {
          navigator.mediaDevices
            .getUserMedia({ audio: false, video: true })
            .then((stream) => {
              video.srcObject = stream;
              localStream = stream;
            })
            .then(window.requestAnimationFrame(processFrame))
            .catch(console.error);

          videoOn = true;
        } else {
          localStream.getTracks()[0].stop();
          videoOn = false;
          canvas.getContext('2d').clearRect(0, 0, canvas.width, canvas.height);
        }
      }
    }

    initialize();

    return () => {
      if (localStream) {
        localStream.getTracks().forEach((track) => track.stop());
      }
    };
  }, []);

  const selectedOptions = (
    <div className="options-column">
      <div className="options-row">
        <ExpandableSection title="Padding">
          <Padding
            onPaddingTypeChange={handlePaddingTypeChange}
          />
        </ExpandableSection>
        <ExpandableSection title="Geometric Spatial Transformations">
          <GeometricTransformations
            onTransformationsChange={handleTransformationsChange}
          />
        </ExpandableSection>
        <ExpandableSection title="Single Pixel Operations">
          <SinglePixelOperations
            onSinglePixelOperationsChange={handleSinglePixelOperationsChange}
          />
        </ExpandableSection>
        <ExpandableSection title="Convolutions">
          <Convolutions
            onConvolutionsChange={handleConvolutionsChange}
            setCustomConvolution={setConvlutionEnabled}
          />
        </ExpandableSection>
      </div>

    </div>
  );

  return (
    <div className="app-container">
      <h1>
        <img src='./logo.png' width="50%" height="auto" alt="IMGPROBOX"/>
      </h1>
      <div className="content">
        <div>
          {selectedOptions}
        </div>
        <div className="options-column">
          <div className="options-row">
            <canvas ref={canvasRef} id="canvas" width="640" height="640" style={{ width: '100%', height: '100%', objectFit: 'contain' }}></canvas>
          </div>
          <div className="options-row">
          </div>
          <div className="options-row">
            <video ref={videoRef} playsInline autoPlay muted style={{ width: '100%' }}></video>
            <button ref={buttonRef} className="btn btn-primary mt-2">Switch webcam</button>
            <input type="file" ref={fileInputRef} id="imageLoader" name="imageLoader" />
          </div>
        </div>
      </div>
    </div>
  );

}

export default App;
