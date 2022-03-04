import numpy as np
import cv2
from math import atan2, pow, sqrt, pi
from utils import *


def click(event, x, y, flags, param):
    global mouseX, mouseY
    if event == cv2.EVENT_LBUTTONUP:
        col = hsv[y, x]
        processed = preprocess(frame)
        blob = find_blob(processed)
        cv2.imshow("frame", processed)

        cv2.imshow("result", result)


frame = cv2.imread("image.jpg")
# frame = cv2.convertScaleAbs(frame, 0, 0.6, 0.3)
frame = adjust_gamma(frame, 0.8)
hsv = cv2.cvtColor(frame, cv2.COLOR_BGR2HSV)
# hsv = cv2.GaussianBlur(hsv, (9, 9), 0)

cv2.namedWindow("frame")
cv2.namedWindow("result")
cv2.setMouseCallback("frame", click)
cv2.imshow("frame", frame)
cv2.imshow("result", result)
cv2.waitKey(0)
cv2.destroyAllWindows()
