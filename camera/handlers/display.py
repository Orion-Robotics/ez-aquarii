import json

import cv2
import msgpack
import numpy as np
from lib.ipc import IPC

from . import BaseFrameHandler
from .constants import *
from .utils import *

thresholds = [
    [255, 0, 255, 0, 255, 0],
    [255, 0, 255, 0, 255, 0],
    [255, 0, 255, 0, 255, 0],
]
names = ["HH", "HL", "VH", "VL", "SH", "SL"]

# ball, goal1, goal2
current = 0


def HH(val):
    thresholds[current][0] = val


def HL(val):
    thresholds[current][1] = val


def SH(val):
    thresholds[current][2] = val


def SL(val):
    thresholds[current][3] = val


def VH(val):
    thresholds[current][4] = val


def VL(val):
    thresholds[current][5] = val


fns = [HH, HL, VH, VL, SH, SL]


def refresh(hsv: np.ndarray):
    m = mask(hsv, thresholds[current])
    b = find_blob(hsv, thresholds[current])
    draw(m, b, center=(300, 300))
    hsvshow("mask", m)
    print(loc(b, center=(300, 300)))


def refreshslider():
    for i in range(6):
        cv2.setTrackbarPos(names[i], "joe", thresholds[current][i])


def hsvshow(name, im):
    cv2.imshow(name, cv2.cvtColor(im, cv2.COLOR_HSV2BGR))


def CE(val):
    global current
    current = val
    refreshslider()


class DisplayHandler(BaseFrameHandler):
    def __init__(self, ipc: IPC | None) -> None:
        super().__init__()
        # self.thresholds = json.loads(path)["thresholds"]
        self.ipc = ipc
        cv2.namedWindow("meow", cv2.WINDOW_NORMAL)
        for i in range(6):
            cv2.createTrackbar(names[i], "meow", 0, 255, fns[i])
        cv2.createTrackbar("current", "meow", 0, 2, CE)

    def handle_frame(self, frame: np.ndarray) -> np.ndarray:
        im = preprocess(frame)
        bl = np.zeros(frame.shape[:2], dtype="uint8")
        bl = cv2.circle(bl, (mw, mh), mw, 255, -1)
        cr = cv2.bitwise_and(im, im, mask=bl)
        # blob = find_blob(im, rgbhsv(70, 30, 40))
        blob = find_blob(cr, thresholds[current])
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

        cv2.imshow("meow", mask(frame, thresholds[current]))
        cv2.waitKey(1)
        return im
