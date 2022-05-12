import json

import cv2
import msgpack
import numpy as np
from lib.ipc import IPC

from . import BaseFrameHandler
from .constants import *
from .utils import *

# thresholds = [
#     [255, 0, 255, 0, 255, 0],
#     [255, 0, 255, 0, 255, 0],
#     [255, 0, 255, 0, 255, 0],
# ]


class DisplayHandler(BaseFrameHandler):
    def __init__(self, ipc: IPC | None, enable_window: bool) -> None:
        super().__init__()
        jsonfile = open(path, "r")
        self.thresholds = json.load(jsonfile)["thresholds"]
        jsonfile.close()
        self.ipc = ipc
        self.enable_window = enable_window
        self.names = ["HH", "HL", "VH", "VL", "SH", "SL"]
        self.fns = [self.HH, self.HL, self.VH, self.VL, self.SH, self.SL]
        # ball, goal1, goal2
        self.current = 0

        if enable_window:
            cv2.namedWindow("meow", cv2.WINDOW_NORMAL)
            for i in range(6):
                cv2.createTrackbar(self.names[i], "meow", 0, 255, self.fns[i])
            cv2.createTrackbar("current", "meow", 0, 2, self.CE)

    def HH(self, val):
        self.thresholds[self.current][0] = val
        self.refresholds()

    def HL(self, val):
        self.thresholds[self.current][1] = val
        self.refresholds()

    def SH(self, val):
        self.thresholds[self.current][2] = val
        self.refresholds()

    def SL(self, val):
        self.thresholds[self.current][3] = val
        self.refresholds()

    def VH(self, val):
        self.thresholds[self.current][4] = val
        self.refresholds()

    def VL(self, val):
        self.thresholds[self.current][5] = val
        self.refresholds()

    def refresh(self, hsv: np.ndarray):
        m = mask(hsv, self.thresholds[self.current])
        b = find_blob(hsv, self.thresholds[self.current])
        draw(m, b, center=(300, 300))
        self.hsvshow("mask", m)
        print(loc(b, center=(300, 300)))

    def refreshslider(self):
        for i in range(6):
            cv2.setTrackbarPos(self.names[i], "joe", self.thresholds[self.current][i])

    def hsvshow(name, im):
        cv2.imshow(name, cv2.cvtColor(im, cv2.COLOR_HSV2BGR))

    def CE(self, val):
        self.current = val
        self.refreshslider()

    def refresholds(self):
        jsonfile = open(path, "w")
        json.dump(jsonfile)
        jsonfile.close()

    def handle_frame(self, frame: np.ndarray) -> np.ndarray:
        im = preprocess(frame)
        bl = np.zeros(frame.shape[:2], dtype="uint8")
        bl = cv2.circle(bl, (mw, mh), mw, 255, -1)
        cr = cv2.bitwise_and(im, im, mask=bl)
        # blob = find_blob(im, rgbhsv(70, 30, 40))
        blob = find_blob(cr, self.thresholds[self.current])
        if blob is not None:
            draw(cr, blob)

        location = loc(blob, center=(mw, mh))
        print(location)
        if location is not None and self.ipc is not None:
            (angle, distance, x, y) = location
            self.ipc.send_data(
                msgpack.packb(
                    {
                        "angle": angle,
                        "distance": distance,
                    }
                )
            )

        if self.enable_window:
            cv2.imshow("meow", mask(frame, self.thresholds[self.current]))
            cv2.waitKey(1)
        return im
