import React, { useEffect, useRef } from 'react';
import 'bootstrap/dist/css/bootstrap.min.css';
import init, { draw } from '../../../pkg/without_a_bundler'; // Replace with the correct import

function App() {
  const canvasRef = useRef();
  const videoRef = useRef();
  const fileInputRef = useRef();
  const buttonRef = useRef();
  let videoOn = false;

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

        draw(canvas.getContext('2d'), 640, 640);

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

      function handleImage(e) {
        var reader = new FileReader();
        const img = new Image();
        reader.onload = function (event) {
          img.src = event.target.result;
          img.onload = () => {
            // Set canvas dimensions to match the uploaded image
            canvas.width = img.width;
            canvas.height = img.height;

            canvas.getContext('2d').drawImage(img, 0, 0);
            console.log(canvas.width, canvas.height);
            draw(canvas.getContext('2d'), canvas.width, 640);
          };
        };
        reader.readAsDataURL(e.target.files[0]);
      }

      //draw(canvas.getContext('2d'), 640, 640);
    }

    initialize();

    return () => {
      if (localStream) {
        localStream.getTracks().forEach((track) => track.stop());
      }
    };
  }, []);

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


}

export default App;

