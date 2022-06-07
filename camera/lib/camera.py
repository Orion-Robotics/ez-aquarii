from time import sleep, time

import cv2
import numpy as np
from handlers import BaseFrameHandler
from handlers.constants import *
from picamera import PiCamera
from picamera.array import PiRGBArray
from threading import Condition
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
        self.frame_cond = Condition()
        self.frames = 0
        self.last_time = time()
        self.handler = handler
        self.frame = None
        self.camera = PiCamera(
            framerate=framerate, resolution=resolution
        )
        self.camera.framerate = framerate
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
        if enable_ipc:
            self.ipc = new_fifo_ipc(ipc_path)

    def run(self):
        last_process_time = time()
        processed_frame_count = 0
        while True:
            with self.frame_cond:
                self.frame_cond.wait()
                if time() - last_process_time > 1:
                    print(f"FPS for processing: {processed_frame_count}")
                    processed_frame_count = 0
                    last_process_time = time()
                if self.frame is not None:
                    self.handler.handle_frame(self.frame)
                    processed_frame_count += 1

    def stop(self):
        self.camera.stop_recording()

    def write(self, buf: bytes):
        self.frames += 1
        if time() - self.last_time > 1:
            print(f"FPS: {self.frames}")
            # print(self.camera.exposure_speed) #+ " " + self.camera.shutter_speed)
            self.frames = 0
            self.last_time = time()
        image = np.frombuffer(buf, dtype=np.uint8).reshape(
            w, h, 3
        )  # this HAS to be width first or else stripes appear
        with self.frame_cond:
            self.frame = image
            self.frame_cond.notify_all()
