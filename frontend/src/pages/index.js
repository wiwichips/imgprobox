import React, { useEffect, useRef, useState, useCallback } from 'react';
import 'bootstrap/dist/css/bootstrap.min.css';
import init, { draw } from '../../../pkg/without_a_bundler'; // Replace with the correct import
import ExpandableSection from './components/ExpandableSection';
import Padding from './components/Padding';
import GeometricTransformations from './components/GeometricTransformations';
import SinglePixelOperations from './components/SinglePixelOperations';
import Convolutions from './components/Convolutions';
import Filtering from './components/Filtering';
import Tab from 'react-bootstrap/Tab';
import Tabs from 'react-bootstrap/Tabs';

function App() {
  const canvasRef = useRef();
  const canvasDrawRef = useRef();
  const canvasImgRef = useRef();
  const videoRef = useRef();
  const fileInputRef = useRef();
  const buttonRef = useRef();
  const hiddenFileInputRef = useRef(); // delete later
  let videoOn = false;
  const cachedImageRef = useRef();

  const [activeTab, setActiveTab] = useState("image");

  const handleWasmDrawRef = useRef();

  // padding
  const [selectedPaddingType, setSelectedPaddingType] = useState('reflected');

  const handlePaddingTypeChange = (paddingType) => {
    setSelectedPaddingType(paddingType);
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

  // filtering
  const [filtering, setFiltering] = useState({
    filterType: 'none',
    neighborhoodSize: 1,
    neighborhoodType: 'chessboard',
    pepper: 0,
    salt: 0,
  });

  const handleFilteringChange = (newFiltering) => {
    setFiltering(newFiltering);
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
    if (singlePixelOperations.sepia) {
      spo_array.push({op_type: 'sepia', a: 0, b: 0});
    } if (singlePixelOperations.grayscale) {
      spo_array.push({op_type: 'grayscale', a: 0, b: 0});
    } if (singlePixelOperations.threshold) {
      spo_array.push({op_type: 'threshold', a: singlePixelOperations.thresholdValue, b: 0});
    } if (singlePixelOperations.linearMapping) {
      spo_array.push({op_type: 'linear', a: singlePixelOperations.linearA, b: singlePixelOperations.linearB});
    } if (singlePixelOperations.powerLaw) {
      spo_array.push({op_type: 'powerLaw', a: singlePixelOperations.gamma, b: 0});
    } if (singlePixelOperations.equalize) {
      spo_array.push({op_type: 'histogram_equalization', a: 0, b: 0});
    }

    console.log(filtering)

    draw(
      // canvas
      canvasObj,
      canvasWidth,
      canvasHeight,
      options,
      // convolution
      kernel,
      // single pixel operations
      spo_array,
      // geometric spatial transformations
      transformations.rotate,
      transformations.scale/100,
      transformations.mirror,
      transformations.flip,
      [],
      transformations.scalingMethod,
      // filtering
      filtering.salt/100,
      filtering.pepper/100,
      filtering.filterType,
      filtering.neighborhoodSize,
      filtering.neighborhoodType,
      // padding
      selectedPaddingType,
      // random
      parseInt(Math.random()*1000),
      );
  }, [singlePixelOperations, convolutions, transformations, selectedPaddingType, filtering]);

  useEffect(() => {
    handleWasmDrawRef.current = handleWasmDraw;
  }, [handleWasmDraw]);

  const handleReDrawImage = useCallback(() => {
    if (!cachedImageRef.current) {
      return;
    }

    const canvas = canvasImgRef.current;
    const ctx = canvas.getContext('2d');
    ctx.drawImage(cachedImageRef.current, 0, 0);

    const canvasDraw = canvasDrawRef.current;
    if (canvasDraw.getContext('2d')['drawImage'] != undefined) {
      canvasDraw.width = canvas.width;
      canvasDraw.height = canvas.height;
      canvasDraw.getContext('2d').drawImage(canvas, 0, 0);
      handleWasmDrawRef.current(canvasDraw.getContext('2d'), canvasDraw.width, canvasDraw.height);
    }
  }, [canvasImgRef, canvasDrawRef, handleWasmDrawRef]);

  function handleImage(e) {
    var reader = new FileReader();
    const img = new Image();
    img.onload = () => {
      // Set canvas dimensions to match the uploaded image
      const canvas = canvasImgRef.current;
      canvas.width = img.width;
      canvas.height = img.height;

      canvas.getContext('2d').drawImage(img, 0, 0);
      cachedImageRef.current = img; // Cache the image
      handleReDrawImage();
    };
    reader.onload = function (event) {
      img.src = event.target.result;
    };
    reader.readAsDataURL(e.target.files[0]);
  }

  // Add a new useEffect to listen for changes in singlePixelOperations, convolutions, and transformations
  useEffect(() => {
    handleReDrawImage();
  }, [singlePixelOperations, convolutions, transformations, handleReDrawImage, selectedPaddingType, filtering]);

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
        handleWasmDrawRef.current(canvas.getContext('2d'), canvas.width, canvas.height); // call wasm
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

  return (
    <div className="app-container">
      <h1>
        <img src='./logo.png' width="100%" height="auto" alt="IMGPROBOX"/>
      </h1>
      <div className="content">
        <div className="options-column">
          <div className="options-row">
            <ExpandableSection title="Padding">
              <Padding
                onPaddingTypeChange={handlePaddingTypeChange}
              />
            </ExpandableSection>
            <ExpandableSection title="Spatial Transformations">
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
            <ExpandableSection title="Filtering">
              <Filtering
                onFilteringChange={handleFilteringChange}
              />
            </ExpandableSection>
          </div>

        </div>
        <div className="options-column">
          <div className="options-row">
            <Tabs
              defaultActiveKey="image"
              id="uncontrolled-tab-example"
              onSelect={(k) => setActiveTab(k)}
            >
            <Tab eventKey="image" title="Image" className='coloredTab'>
              <div className="image-column">
                <div>
                  <input type="file" ref={fileInputRef} id="imageLoader" name="imageLoader" className={activeTab === "webcam" ? "hideMe" : ""}/>
                </div>
                <canvas ref={canvasDrawRef} id="canvasUpdate" width="640" height="640" style={{ width: '100%', height: '100%', objectFit: 'contain' }}></canvas>
                <canvas ref={canvasImgRef} id="canvas" width="640" height="640" style={{ width: '100%', height: '100%', objectFit: 'contain' }}></canvas>
                <video ref={videoRef} playsInline autoPlay muted style={{ width: '100%' }} className={activeTab === "webcam" ? "": "hideMe"}></video>
              </div>
              <div>
                <button ref={buttonRef} className="btn btn-primary mt-2 hideMe">Switch webcam</button>
              </div>
            </Tab> 
            <Tab eventKey="webcam" title="Webcam" className='coloredTab'>
              <div className="image-column">
                <div>
                  <button ref={buttonRef} className="btn btn-primary mt-2">Switch Webcam On/Off</button>
                </div>
                <div>
                </div>
                <canvas ref={canvasRef} id="canvas" width="640" height="640" style={{ width: '100%', height: '100%', objectFit: 'contain' }}></canvas>
                <video ref={videoRef} playsInline autoPlay muted style={{ width: '100%' }} ></video>
              </div>
            </Tab> 
            </Tabs>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
