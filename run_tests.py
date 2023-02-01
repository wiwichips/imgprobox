#!/usr/bin/env python3
import toolbox
import sys

filename = './pics/otter.png'
if (len(sys.argv) > 1):
    filename = sys.argv[1]
print(f"Trying to manipulate {filename}")

print("crop")
img_op = toolbox.ImageOperation(filename)


