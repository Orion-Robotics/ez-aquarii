from threading import Thread

import cv2
import numpy as np
from picamera import PiCamera
from picamera.array import PiRGBArray

from handlers import BaseFrameHandler
from handlers.display import DisplayHandler
from lib.ipc import IPC, new_fifo_ipc


class Camera:
    def __init__(
        self,
        handler: BaseFrameHandler,
        resolution=(640, 480),
        framerate=60,
        enable_ipc=False,
        ipc_path="./camera",
    ):
        self.handler = handler

        self.camera = PiCamera()
        self.camera.resolution = resolution
        self.camera.framerate = framerate
        self.raw_capture = PiRGBArray(self.camera, size=resolution)
        self.stream = self.camera.capture_continuous(
            self.raw_capture,
            format="bgr",
            use_video_port=True,
        )
        self.frame = None
        self.stopped = False
        Thread(target=self.handle_stream, args=()).start()
        if enable_ipc:
            self.ipc = new_fifo_ipc(ipc_path)

    def stop(self):
        self.stopped = True

    def handle_stream(self):
        for frame in self.stream:
            self.frame = frame.array
            self.handler.handle_frame(self.frame)
            self.raw_capture.truncate(0)
            if self.stopped:
                self.stream.close()
                self.camera.close()
                self.raw_capture.close()


if __name__ == "__main__":
    cam = Camera(DisplayHandler())
