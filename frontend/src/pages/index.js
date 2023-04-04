import React, { useEffect, useRef, useState, useCallback } from 'react';
import 'bootstrap/dist/css/bootstrap.min.css';
import init, { draw } from '../../../pkg/without_a_bundler'; // Replace with the correct import

function App() {
  const canvasRef = useRef();
  const videoRef = useRef();
  const fileInputRef = useRef();
  const buttonRef = useRef();
  let videoOn = false;

  const handleWasmDrawRef = useRef();


  const [convolutionDemo, setConvolutionDemo] = useState(false);
  const [powerLawMappingDemo, setPowerLawMappingDemo] = useState(false);
  const [inverseDemo, setInverseDemo] = useState(false);
  const [stackedDemo, setStackedDemo] = useState(false);

  // The corresponding onChange handlers for the checkboxes
  const handleConvolutionDemoChange = (e) => {
    setConvolutionDemo(e.target.checked);
  };

  const handlePowerLawMappingDemoChange = (e) => {
    setPowerLawMappingDemo(e.target.checked);
  };

  const handleInverseDemoChange = (e) => {
    setInverseDemo(e.target.checked);
  };

  const handleStackedDemoChange = (e) => {
    setStackedDemo(e.target.checked);
  };

  /*
  function handleWasmDraw(canvasObj, canvasWidth, canvasHeight) {
    const options = [
        convolutionDemo ? 'convolutionDemo' : null,
        powerLawMappingDemo ? 'powerLawMappingDemo' : null,
        inverseDemo ? 'inverseDemo' : null,
        stackedDemo ? 'stackedDemo' : null,
      ].filter((option) => option !== null);

    console.log(options);

    // Call the draw function from wasm
    draw(canvasObj, canvasWidth, canvasHeight, options);
  }
  */
  const handleWasmDraw = useCallback((canvasObj, canvasWidth, canvasHeight) => {
    const options = [
      convolutionDemo ? 'convolutionDemo' : null,
      powerLawMappingDemo ? 'powerLawMappingDemo' : null,
      inverseDemo ? 'inverseDemo' : null,
      stackedDemo ? 'stackedDemo' : null,
    ].filter((option) => option !== null);

    //console.log(options);

    // Call the draw function from wasm
    draw(canvasObj, canvasWidth, canvasHeight, options);
  }, [convolutionDemo, powerLawMappingDemo, inverseDemo, stackedDemo]);

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
        canvas.width = video.videoWidth;
        canvas.height = video.videoHeight;

        canvas.getContext('2d').drawImage(video, 0, 0, canvas.width, canvas.height);

        let image = new Image();
        image.src = canvas.toDataURL();
        canvas.getContext('2d').drawImage(image, 0, 0);

        //draw(canvas.getContext('2d'), 640, 640);
        handleWasmDrawRef.current(canvas.getContext('2d'), 640, 640);

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

/*
return (
  <div className="container-fluid full-height">
    <div className="row full-height">
      <div className="col-4 bg-primary"></div>
      <div className="col-4 bg-secondary">
        <input type="file" ref={fileInputRef} id="imageLoader" name="imageLoader" />
        <label>Image File:</label>
        <br />
        <canvas ref={canvasRef} id="canvas" width="640" height="640" style={{ width: '100%' }}></canvas>
      </div>
      <div className="col-4 bg-primary"></div>
    </div>
    <div className="row full-height">
      <div className="col-4 bg-success"></div>
      <div className="col-4 bg-info">
        <video ref={videoRef} playsInline autoPlay muted style={{ width: '100%' }}></video>
        <button ref={buttonRef} className="btn btn-primary mt-2">Switch webcam</button>
      </div>
      <div className="col-4 bg-success"></div>
    </div>
  </div>
);
*/
return (
  <div className="container-fluid full-height">
    <div className="row full-height">
    <div className="col-4 bg-primary d-flex flex-column align-items-center justify-content-center">
      <div className="form-check">
        <input
          className="form-check-input"
          type="checkbox"
          value={convolutionDemo}
          onChange={(e) => setConvolutionDemo(e.target.checked)}
          id="convolutionDemo"
        />
        <label className="form-check-label" htmlFor="convolutionDemo">
          Convolution Demo
        </label>
      </div>
      <div className="form-check">
        <input
          className="form-check-input"
          type="checkbox"
          value={powerLawMappingDemo}
          onChange={(e) => setPowerLawMappingDemo(e.target.checked)}
          id="powerLawMappingDemo"
        />
        <label className="form-check-label" htmlFor="powerLawMappingDemo">
          Power Law Mapping Demo
        </label>
      </div>
      <div className="form-check">
        <input
          className="form-check-input"
          type="checkbox"
          value={inverseDemo}
          onChange={(e) => setInverseDemo(e.target.checked)}
          id="inverseDemo"
        />
        <label className="form-check-label" htmlFor="inverseDemo">
          Inverse Demo
        </label>
      </div>
      <div className="form-check">
        <input
          className="form-check-input"
          type="checkbox"
          value={stackedDemo}
          onChange={(e) => setStackedDemo(e.target.checked)}
          id="stackedDemo"
        />
        <label className="form-check-label" htmlFor="stackedDemo">
          Stacked Demo
        </label>
      </div>
    </div>
      <div className="col-4 bg-secondary square-cell">
        <div className="square-content">
          <canvas ref={canvasRef} id="canvas" width="640" height="640" style={{ width: '100%', height: '100%', objectFit: 'contain' }}></canvas>
        </div>
      </div>
    </div>
    <div className="row full-height">
      <div className="col-4 bg-success d-flex align-items-center justify-content-center">
        <input type="file" ref={fileInputRef} id="imageLoader" name="imageLoader" />
      </div>
      <div className="col-4 bg-info">
        <video ref={videoRef} playsInline autoPlay muted style={{ width: '100%' }}></video>
        <button ref={buttonRef} className="btn btn-primary mt-2">Switch webcam</button>
      </div>
    </div>
  </div>
);

}

export default App;

