import os
from time import time

import cv2
import numpy as np
from picamera import PiCamera
from picamera.array import PiRGBArray

from handlers import BaseFrameHandler, constants
from handlers.constants import *
from handlers.display import DisplayHandler
from handlers.noop import NoopHandler
from lib.ipc import new_fifo_ipc
from lib.streaming import StreamingFrameHandler


class Camera:
    def __init__(
        self,
        handler: BaseFrameHandler,
        resolution=(h, w),  # this HAS to be height first or else stripes appear
        framerate=90,
        enable_ipc=False,
        ipc_path="./camera",
    ):
        self.frames = 0
        self.last_time = time()
        self.handler = handler
        self.frame = None
        self.camera = PiCamera(
            sensor_mode=5, framerate=framerate, resolution=resolution
        )
        self.camera.resolution = resolution
        self.camera.framerate = framerate
        self.raw_capture = PiRGBArray(self.camera, size=resolution)
        self.camera.start_recording(
            self,
            format="bgr",
        )
        self.frame = None
        self.stopped = False
        if enable_ipc:
            self.ipc = new_fifo_ipc(ipc_path)

    def run(self):
        while True:
            if self.frame is not None:
                self.handler.handle_frame(self.frame)

    def stop(self):
        self.stopped = True
        self.camera.stop_recording()

    def write(self, buf: bytes):
        self.frames += 1
        if time() - self.last_time > 1:
            print(f"FPS: {self.frames}")
            self.frames = 0
            self.last_time = time()
        image = np.frombuffer(buf, dtype=np.uint8).reshape(
            w, h, 3
        )  # this HAS to be width first or else stripes appear
        image = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)
        self.frame = image


if __name__ == "__main__":
    try:
        handler = DisplayHandler()
        handler = StreamingFrameHandler(handler, constants.SERVER_ADDRESS)
        cam = None
        cam = Camera(handler)
        cam.run()
    except SystemExit:
        os._exit(0)
