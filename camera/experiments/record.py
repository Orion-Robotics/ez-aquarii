from time import time, sleep
import picamera
import numpy as np
import cv2

class op(object):
    def __init__(self):
        self.t = time()
    def write(self, buf):
        print(1/(time()-self.t))
        self.t = time()
        data = np.frombuffer(buf, dtype=np.uint8, count=128*96)
        cv2.imshow("amogus", data)
    def flush(self):
        pass
with picamera.PiCamera(sensor_mode=7, resolution = '640x480', framerate=90) as camera:
    sleep(2)
    output = op()
    camera.start_recording(output, 'rgb')
    camera.wait_recording(1)
    camera.stop_recording()
