from time import time, sleep
import traceback
import cv2
import numpy as np

from handlers import BaseFrameHandler, constants
from handlers.constants import *
from handlers.utils import *
from handlers.display import DisplayHandler
from handlers.noop import NoopHandler
from lib.streaming import StreamingFrameHandler


def HH(val):
    thresholds[0] = val
    refresh()


def HL(val):
    thresholds[1] = val
    refresh()


def SH(val):
    thresholds[2] = val
    refresh()


def SL(val):
    thresholds[3] = val
    refresh()


def VH(val):
    thresholds[4] = val
    refresh()


def VL(val):
    thresholds[5] = val
    refresh()


def refresh():
    m = mask(hsv, thresholds)
    b = find_blob(hsv, thresholds)
    draw(m, b, center=(300, 300))
    hsvshow("mask", m)
    print(loc(b, center=(300, 300)))


def hsvshow(name, im):
    cv2.imshow(name, cv2.cvtColor(im, cv2.COLOR_HSV2BGR))


thresholds = [255, 0, 255, 0, 255, 0]
joe = cv2.imread("cha.jpg")
joe = cv2.resize(joe, (600, 600))
hsv = cv2.cvtColor(joe, cv2.COLOR_BGR2HSV)
hsvshow("joe", hsv)
cv2.createTrackbar("HH", "joe", 0, 255, HH)
cv2.createTrackbar("HL", "joe", 0, 255, HL)
cv2.createTrackbar("VH", "joe", 0, 255, VH)
cv2.createTrackbar("VL", "joe", 0, 255, VL)
cv2.createTrackbar("SH", "joe", 0, 255, SH)
cv2.createTrackbar("SL", "joe", 0, 255, SL)
cv2.waitKey(0)
cv2.destroyAllWindows()
