# ![IMGPROBOX](https://wiwichips.github.io/imgprobox/logo.png)
➜ https://wiwichips.github.io/imgprobox/

IMGPROBOX is an online image processing toolbox written in Rust + WASM with a React + Next.js frontend.


## User Manual

### General Use

In general, image processing options are set on the left hand side within the expandable sections, the image on the bottom of the right hand side is the original image, and the image on the top of the right side is the modified image.

A user can toggle between two modes (Image, and Webcam). When on Image, it will allow a user to upload an image by pressing the "browse" button. When on Webcam, it will display the live video feed of the webcam on the right hand side.

Whenever a button is pressed, the image processing option will be applied. Processing options can be undone by deselecting them most of the time.


### Padding

Any operations that use padding can be modified by clicking on a different radio button for padding types. For instance, clicking on "circular indexing" radio button will result in circular indexing being used for all operations that require some use of padding.


### Spatial Transformations

When enabling the "Crop Image" checkbox, a rectangle will appear over the image and the user can drag it out over the image to highlight a part they would like to crop. Then they click the "crop" indicator button and the image will then be cropped.

Checking the mirror box will flip the image horizontally. 

Checking the "flip upside down" box will flip the image vertically.

The rotation degree is entered using a slider, the user will adjust the slider to an appropriate position to view the rotation on the image.

Scaling is accomplished by allowing the user to pick one of two different interpolation methods (bi-cubic is not working and will be removed) and allows the user to enter in a percent value for the image to be scaled by.


### Single Pixel Operations

A built in operation "inverse" can be enabled by checking the checkbox for it.

An image threshold can be entered by specifying a gray level value where all values below will be set to 0 and all values above will be set to 255.

Linear mappings allow a gain and bias to be entered and will take effect when the checkbox is checked.

Power Law Mappings allow a gamma value to be set and will take effect when the checkbox is checked.

Histogram equalization is enabled when checked.


### Convolutions

A user can specify a custom kernel for convolutions by specifying any width and height and entering the values into the box. Then they click "enable" and "set kernel" for it to display the processed image on the right. If they click "normalize" it will normalize the values to 1.

A couple useful built in kernels are provided for convolutions which can be applied by clicking on the radio button and pressing the "set kernel" button.


### Filtering

Salt and peppering can be added to the image by specifying a percentage amount of the image to be salted and a specific amount to be peppered by. 

The image can be denoised by specifying a neighbourhood type by clicking on the radio button associated with the specific neighbourhood and a distance for it to look at neighbourhood pixels. 


## Installation
Ensure cargo and npm are installed on your system. Run `build.sh && cd frontend && npm i` script to install required reach and rust packages requied for this project.

## Deployment instructions
Run the `deploy.sh` script in the root directory of this repository. Then serve the files from the `frontend/out` folder.
- `./deploy.sh`
- `ls frontend/out/`

## Testing instructions
Running locally
- To run IMGPROBOX locally, just run the `./run.sh` script.

Unit Tests
- Currently there are no unit tests. If you would like to add some, please make a pull request.

