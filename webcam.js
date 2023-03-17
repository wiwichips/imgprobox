// upload image
var imageLoader = document.getElementById('imageLoader');
imageLoader.addEventListener('change', handleImage, false);
var ctx = canvas.getContext('2d');

const video = document.querySelector('video');
const button = document.querySelector('button');

function processFrame() {
  /* set the canvas to the dimensions of the video feed */
  canvas.width = video.videoWidth;
  canvas.height = video.videoHeight;
  /* make the snapshot */
  canvas.getContext('2d').drawImage(video, 0, 0, canvas.width, canvas.height);
    //extra
    let image = new Image();
    image.src = canvas.toDataURL();
    crazyColour(image);
    if (videoOn === true)
        window.requestAnimationFrame(processFrame);
};

var videoOn = false;
var localStream;
function flipVideoFeed() {
    if (videoOn === false) {
        navigator.mediaDevices.getUserMedia( {audio: false, video: true })
            .then((stream) => {video.srcObject = stream; localStream = stream;})
            .then(window.requestAnimationFrame(processFrame))
            .catch(error => console.error(error)); 
        videoOn = true;
        loop = 2;
    } else {
        localStream.getTracks()[0].stop();
        videoOn = false;
        ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
    }
}

button.onclick = flipVideoFeed;

function handleImage(e){
    var reader = new FileReader();
    const img = new Image();
    reader.onload = function(event){
        img.src = event.target.result;
        img.onload = () => {
            crazyColour(img)
        }
    }
    reader.readAsDataURL(e.target.files[0]);     
}

