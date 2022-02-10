import cv2
import numpy as np
from .utils import *
from .constants import *

from . import BaseFrameHandler


class DisplayHandler(BaseFrameHandler):
    def __init__(self) -> None:
        super().__init__()
        # cv2.namedWindow("trolling", cv2.WINDOW_NORMAL)

    def handle_frame(self, frame: np.ndarray):
        ke = mask(frame, (100, 100, 100))
        cv2.imshow("trolling", ke)
        cv2.waitKey(1)
        
