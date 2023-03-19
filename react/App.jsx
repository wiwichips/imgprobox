import React, { useEffect, useRef, useState, useCallback } from 'react';
import './App.css';

function App() {
  const videoRef = useRef(null);
  const canvasRef = useRef(null);
  const [webcamEnabled, setWebcamEnabled] = useState(true);
  const [uploadedImage, setUploadedImage] = useState(null);
  const [paddingBottom, setPaddingBottom] = useState('75%');

  const processFrame = useCallback(() => {
    if (!canvasRef.current) {
      return;
    }

    const canvas = canvasRef.current;
    const ctx = canvas.getContext('2d');

    if (webcamEnabled && videoRef.current) {
      canvas.width = videoRef.current.videoWidth;
      canvas.height = videoRef.current.videoHeight;
      ctx.drawImage(videoRef.current, 0, 0, canvas.width, canvas.height);
    } else if (!webcamEnabled && uploadedImage) {
      canvas.width = uploadedImage.width;
      canvas.height = uploadedImage.height;
      ctx.drawImage(uploadedImage, 0, 0, canvas.width, canvas.height);

      const newPaddingBottom = `${(canvas.height / canvas.width) * 100}%`;
      if (newPaddingBottom !== paddingBottom) {
        setPaddingBottom(newPaddingBottom);
      }
    }

    window.requestAnimationFrame(processFrame);
  }, [webcamEnabled, uploadedImage, paddingBottom]);

  const setupWebcam = useCallback(async () => {
    if (navigator.mediaDevices && navigator.mediaDevices.getUserMedia) {
      try {
        const stream = await navigator.mediaDevices.getUserMedia({ video: true });
        videoRef.current.srcObject = stream;
        videoRef.current.addEventListener('loadedmetadata', () => {
          videoRef.current.play();
          processFrame();
        });
      } catch (err) {
        console.error('Error accessing webcam: ', err);
      }
    }
  }, [processFrame]);

  useEffect(() => {setupWebcam();}, [webcamEnabled, uploadedImage, setupWebcam]);

  function handleImageUpload(e) {
    const file = e.target.files[0];
    if (file) {
      const reader = new FileReader();
      reader.onload = (event) => {
        const image = new Image();
        image.src = event.target.result;
        image.onload = () => {
          setUploadedImage(image);
        };
      };
      reader.readAsDataURL(file);
    }
  }

  function toggleWebcam() {
    setWebcamEnabled(!webcamEnabled);
  }

  return (
    <div className="container">
      <header className="header">Header</header>
      <div className="columns">
        <div className="column">
          <h2>Column 1</h2>
          <p>Content for Column 1</p>
          <button>Button 1</button>
        </div>
        <div className="column column-2">
          <h2>Column 2</h2>
          <video ref={videoRef} style={{ display: 'none' }}></video>
          <div className="canvas-container" style={{ paddingBottom }}>
            <canvas ref={canvasRef}></canvas>
          </div>
          <button onClick={toggleWebcam}>
            {webcamEnabled ? 'Show Uploaded Image' : 'Show Webcam'}
          </button>
          <br />
          <input
            type="file"
            accept="image/*"
            onChange={handleImageUpload}
            style={{ display: webcamEnabled ? 'none' : 'inline' }}
          />
        </div>
        <div className="column">
          <h2>Column 3</h2>
          <p>Content for Column 3</p>
          <button>Button 3</button>
        </div>
      </div>
    </div>
  );
}

export default App;

