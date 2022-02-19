import cv2
import numpy as np

from . import BaseFrameHandler


class DisplayHandler(BaseFrameHandler):
    def handle(self, frame: np.ndarray):
        cv2.imshow("trolsdfsdfling", frame)
        # pass
