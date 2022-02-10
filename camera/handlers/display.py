import cv2
import numpy as np
from ..experiments.utils import *
from ..experiments.constants import *

from . import BaseFrameHandler


class DisplayHandler(BaseFrameHandler):
    def __init__(self) -> None:
        super().__init__()
        # cv2.namedWindow("trolling", cv2.WINDOW_NORMAL)

    def handle_frame(self, frame: np.ndarray):
        mask = mask(frame, (100, 100, 100))
        cv2.imshow("trolling", mask)
        cv2.waitKey(1)
        pass
