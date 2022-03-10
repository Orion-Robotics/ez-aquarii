import os
from threading import Thread
from time import time, sleep

import cv2
import numpy as np
from picamera import PiCamera
from picamera.array import PiRGBArray

camera = PiCamera()
camera.resolution = (640, 480)
camera.framerate = 24
sleep(2)
while True:
    image = np.empty((480*640*3,), dtype=np.uint8)
    camera.capture(image, 'bgr')
    image = image.reshape((480, 640, 3))
    cv2.imshow('s', image)
    cv2.waitKey(0)
cv2.destroyAllWindows()
