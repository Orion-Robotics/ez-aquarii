from time import time, sleep
from copy import copy
import traceback
import cv2
import numpy as np

from handlers import BaseFrameHandler, constants
from handlers.constants import *
from handlers.utils import *
from handlers.display import DisplayHandler
from handlers.noop import NoopHandler
from lib.streaming import StreamingFrameHandler

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
    refresh()


def HL(val):
    thresholds[current][1] = val
    refresh()


def SH(val):
    thresholds[current][2] = val
    refresh()


def SL(val):
    thresholds[current][3] = val
    refresh()


def VH(val):
    thresholds[current][4] = val
    refresh()


def VL(val):
    thresholds[current][5] = val
    refresh()


fns = [HH, HL, VH, VL, SH, SL]


def refresh():
    m = mask(hsv, thresholds[current])
    b = find_blob(hsv, thresholds[current])
    draw(m, b, center=(300, 300))
    hsvshow("mask", label(m, b))
    print(loc(b, center=(300, 300)))


def refreshslider():
    for i in range(6):
        cv2.setTrackbarPos(names[i], "joe", thresholds[current][i])


def hsvshow(name, im):
    cv2.imshow(name, cv2.cvtColor(im, cv2.COLOR_HSV2BGR))


def remap(a):
    return a


def label(im, *blobs):
    out = copy(im)
    for blob in blobs:
        ang, dist, x, y = loc(blob)
        cv2.putText(
            out,
            f"{remap(int(ang))} deg {int(dist)} px",
            org=(x - 100, y),
            fontFace=cv2.FONT_HERSHEY_PLAIN,
            color=(150, 255, 255),
            fontScale=2,
            thickness=2,
        )
    return out


def CE(val):
    global current
    current = val
    refreshslider()


joe = cv2.imread("cha.jpg")
joe = cv2.resize(joe, (600, 600))
hsv = cv2.cvtColor(joe, cv2.COLOR_BGR2HSV)
hsvshow("joe", hsv)
for i in range(6):
    cv2.createTrackbar(names[i], "joe", 0, 255, fns[i])
cv2.createTrackbar("current", "joe", 0, 2, CE)
refreshslider()
cv2.waitKey(0)
cv2.destroyAllWindows()
