import os
import traceback
from time import sleep, time

import cv2
import numpy as np

from handlers import BaseFrameHandler, constants
from handlers.display import DisplayHandler
from handlers.noop import NoopHandler
from handlers.test import TestHandler

from lib.camera import Camera
from lib.streaming import StreamingFrameHandler

if __name__ == "__main__":
    try:
        ipc = IPC()
        handler = DisplayHandler(None, False)
        handler = StreamingFrameHandler(handler, constants.SERVER_ADDRESS)
        #joe = cv2.imread("cha.jpg")
        #joe = cv2.resize(joe, (600, 600))
        #while True:
        #    handler.handle_frame(joe)
        cam = Camera(handler)
        cam.run()
    except Exception as e:
        print(e)
        traceback.print_exc()
        os._exit(0)
