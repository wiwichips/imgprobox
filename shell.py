#!/usr/bin/env python3
import readline
import shlex
import os
import toolbox

img_api = toolbox.API()

while True:
    command = input("\x1b[35mIMG ↦\x1b[0m ")
    cmd_args = shlex.split(command)

    if (len(cmd_args) == 0):
        continue
    elif cmd_args[0] == "exit" or cmd_args[0] == "quit":
        break
    elif hasattr(img_api, cmd_args[0]):
        function = getattr(img_api, cmd_args[0])
        try:
            output = function(cmd_args[1:])
        except Exception as e:
            print("Encountered an error:")
            print(e)
    else:
        os.system(command)


