import os
from time import time, sleep
import cv2
import numpy as np
from picamera import PiCamera
from picamera.array import PiRGBArray
import traceback
from copy import copy
from handlers import BaseFrameHandler, constants
from handlers.constants import *
from handlers.utils import *
from math import sin, cos

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
    # refresh()

def HL(val):
    thresholds[current][1] = val
    # refresh()

def SH(val):
    thresholds[current][2] = val
    # refresh()

def SL(val):
    thresholds[current][3] = val
    # refresh()

def VH(val):
    thresholds[current][4] = val
    # refresh()

def VL(val):
    thresholds[current][5] = val
    # refresh()

fns = [HH, HL, VH, VL, SH, SL]

def refresh():
    m = mask(hsv, thresholds[current], erode=True)
    b = find_optimal_blob(hsv, thresholds[current], ball_heuristic)
    draws(m, thresholds[current])
    draw(m, b, center=(300, 300))
    # hsvshow("mask", label(m, b))
    # hsvshow("res", blackout(hsv, thresholds[current]))
    return b, m

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

joe = cv2.imread("fshot.png")
# joe = cv2.resize(joe, (600, 600))
hsv = cv2.cvtColor(joe, cv2.COLOR_BGR2HSV)
# hsv = cv2.GaussianBlur(hsv, (5, 5), 0)
hsvshow("joe", hsv)
for i in range(6):
    cv2.createTrackbar(names[i], "joe", 0, 255, fns[i])
cv2.createTrackbar("current", "joe", 0, 2, CE)
refreshslider()
# cv2.waitKey(0)
# cv2.destroyAllWindows()
FUT = 20

camera = PiCamera()
camera.resolution = (640, 480)
camera.framerate = 90
camera.sensor_mode = 7
sleep(2)

rs = []
thetas = []
while True:
    image = np.empty((480*640*3,), dtype=np.uint8)
    camera.capture(image, 'bgr')
    image = image.reshape((480, 640, 3))
    hsv = cv2.cvtColor(image, cv2.COLOR_BGR2HSV)
    blob, mask = refresh()
    if blob is not None:
        print(blob)
        bp = blob.loc()
        thetas.append(bp[0])
        rs.append(bp[1])
        if len(rs) > 10:
            rs.pop(0)
            thetas.pop(0)
        xs = np.array(range(len(rs)))
        rcoeffs = np.polyfit(xs, rs, 5)
        rfn = np.poly1d(rcoeffs)
        tcoeffs = np.polyfit(xs, thetas, 5)
        tfn = np.poly1d(tcoeffs)
        futurepos = (rfn(FUT) * cos(tfn(FUT)), rfn(FUT) * sin(tfn(FUT)))
        mask = cv2.circle(mask, futurepos, radius=5, color=(111, 65, 26), thickness=-1)
    hsvshow(mask)
    cv2.waitKey(1)
    # cv2.imshow('hsv', image)
cv2.destroyAllWindows()
