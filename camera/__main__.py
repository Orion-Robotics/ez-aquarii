from threading import Thread
from time import time

import cv2
import numpy as np
from picamera import PiCamera
from picamera.array import PiRGBArray

from handlers import BaseFrameHandler
from handlers.display import DisplayHandler
from lib.ipc import new_fifo_ipc


class Camera:
    def __init__(
        self,
        handler: BaseFrameHandler,
        resolution=(1280, 720),
        framerate=90,
        enable_ipc=False,
        ipc_path="./camera",
    ):
        self.frames = 0
        self.last_time = time()
        self.handler = handler

        self.camera = PiCamera(
            sensor_mode=7, framerate=framerate, resolution=resolution
        )
        self.camera.resolution = resolution
        self.camera.framerate = framerate
        self.raw_capture = PiRGBArray(self.camera, size=resolution)
        self.camera.start_recording(
            self,
            format="yuv",
        )
        self.camera.wait_recording(100)
        self.frame = None
        self.stopped = False
        if enable_ipc:
            self.ipc = new_fifo_ipc(ipc_path)

    def stop(self):
        self.stopped = True

    def write(self, buf):
        self.frames += 1
        if time() - self.last_time > 1:
            print(f"FPS: {self.frames}")
            self.frames = 0
            self.last_time = time()
        # self.frame = frame.array
        # self.handler.handle_frame(self.frame)
        # self.raw_capture.truncate(0)
        # if self.stopped:
        #     self.stream.close()
        #     self.camera.close()
        #     self.raw_capture.close()


if __name__ == "__main__":
    cam = Camera(DisplayHandler())
