from time import sleep, time

import cv2
import numpy as np
from handlers import BaseFrameHandler
from handlers.constants import *
from picamera import PiCamera
from picamera.array import PiRGBArray

from lib.ipc import new_fifo_ipc


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
        self.camera.framerate = framerate
        self.raw_capture = PiRGBArray(self.camera, size=resolution)
        # self.camera.awb_mode = "off"
        # self.camera.iso = 800
        # self.camera.saturation = 0
        self.camera.resolution = resolution
        sleep(2)
        self.camera.exposure_mode = "off"
        self.camera.start_recording(
            self,
            format="bgr",
        )
        self.frame = None
        self.stopped = False
        if enable_ipc:
            self.ipc = new_fifo_ipc(ipc_path)

    def run(self):
        last_process_time = time()
        processed_frame_count = 0
        while True:
            if time() - last_process_time > 1:
                print(f"FPS for processing: {processed_frame_count}")
                processed_frame_count = 0
                last_process_time = time()
            if self.frame is not None:
                self.handler.handle_frame(self.frame)
                processed_frame_count += 1
            else:
                print("No frame")

    def stop(self):
        self.stopped = True
        self.camera.stop_recording()

    def write(self, buf: bytes):
        self.frames += 1
        if time() - self.last_time > 1:
            # print(f"FPS: {self.frames}")
            # print(self.camera.exposure_speed) #+ " " + self.camera.shutter_speed)
            self.frames = 0
            self.last_time = time()
        image = np.frombuffer(buf, dtype=np.uint8).reshape(
            w, h, 3
        )  # this HAS to be width first or else stripes appear
        image = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)
        self.frame = image
