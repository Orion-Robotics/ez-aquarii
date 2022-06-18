import os
from threading import Thread
from time import time, sleep

import cv2
import numpy as np
from picamera import PiCamera
from picamera.array import PiRGBArray

camera = PiCamera()
s = (640, 480)
camera.resolution = s
camera.framerate = 90
camera.sensor_mode=7
rawCapture = PiRGBArray(camera, size=s)
sleep(2)
t = time()
cv2.namedWindow("s")
for frame in camera.capture_continuous(rawCapture, format="bgr", use_video_port=True):
    image = frame.array
    cv2.imshow("frame", image)
    key = cv2.waitKey(1) & 0xFF
    rawCapture.truncate(0)
    # print(1/(t-time()))
    # t = time()
    # if key == ord("q"):
        # break
