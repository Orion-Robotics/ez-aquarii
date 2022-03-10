import cv2
import numpy as np

from . import BaseFrameHandler
from .constants import *
from .utils import *


class DisplayHandler(BaseFrameHandler):
    def __init__(self) -> None:
        super().__init__()
        # cv2.namedWindow("trolling", cv2.WINDOW_NORMAL)

    def handle_frame(self, frame: np.ndarray) -> np.ndarray:
        # ke = mask(frame, (100, 100, 100))
        im = preprocess(frame)
        bl = np.zeros(frame.shape[:2], dtype="uint8")
        bl = cv2.circle(bl, (mw, mh), mw, 255, -1)
        cr = cv2.bitwise_and(im, im, mask=bl)
        blob = find_blob(im, rgbhsv(70, 30, 40))
        draw(cr, blob)
        cv2.imshow("trolling", cr)
        cv2.waitKey(1)
        return cr
