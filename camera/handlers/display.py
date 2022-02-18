import cv2
import numpy as np

class DisplayHandler(BaseFrameHandler):
    def handle(frame: np.array):
        cv2.imshow(frame)
