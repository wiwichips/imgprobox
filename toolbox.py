from PIL import Image
import pdb

class ImageOperation:
    def __init__(self, image_filename):
        self.img = Image.open(image_filename)
        self.pixels = self.img.load()        

    def crop(self):
        img = self.img
        pixels = self.pixels
        p = pixels

        for i in range(img.size[0]): # for every pixel:
            for j in range(img.size[1]):
                r, g, b = pixels[i,j]
                # pixels[i,j] = ((g*i*j)%255, (r%(j+1)+(i%150))%250, min(g//(i+1) * j, 255))
                pixels[i,j] = (r,g,b)
        img.show()


class API:
    def __init__(self):
        pass


    def help(self, _):
        print("Image Processing Toolbox")

