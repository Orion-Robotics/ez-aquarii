import cv2
import numpy as np

from . import BaseFrameHandler


class DisplayHandler(BaseFrameHandler):
    def __init__(self) -> None:
        super().__init__()
        # cv2.namedWindow("trolling", cv2.WINDOW_NORMAL)

    def handle_frame(self, frame: np.ndarray):
        print("woozyyy")
        cv2.imshow("trolling", frame)
        cv2.waitKey(0)
        # cv2.destroyAllWindows()
        # pass
