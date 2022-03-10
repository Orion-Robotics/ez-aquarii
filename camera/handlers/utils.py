import numpy as np
import cv2
from .constants import *
from math import pi, atan2, sqrt, pow


def adjust_gamma(image, gamma=1.0):
    # build a lookup table mapping the pixel values [0, 255] to
    # their adjusted gamma values
    invGamma = 1.0 / gamma
    table = np.array(
        [((i / 255.0) ** invGamma) * 255 for i in np.arange(0, 256)]
    ).astype("uint8")
    # apply gamma correction using the lookup table
    return cv2.LUT(image, table)


def mask(image, target):
    lower = np.array([target[0] - HDIFF, target[1] - SDIFF, target[2] - VDIFF])
    upper = np.array([target[0] + HDIFF, target[1] + SDIFF, target[2] + VDIFF])
    mask = cv2.inRange(image, lower, upper)
    mask = cv2.GaussianBlur(mask, (5, 5), 0)
    return cv2.bitwise_and(image, image, mask=mask)


def find_blob(image, target):
    lower = np.array([target[0] - HDIFF, target[1] - SDIFF, target[2] - VDIFF])
    upper = np.array([target[0] + HDIFF, target[1] + SDIFF, target[2] + VDIFF])
    mask = cv2.inRange(image, lower, upper)
    mask = cv2.GaussianBlur(mask, (5, 5), 0)
    contours, _ = cv2.findContours(mask, cv2.RETR_TREE, cv2.CHAIN_APPROX_NONE)
    try: 
        blob = max(contours, key=lambda el: cv2.contourArea(el))
    except:
        return None
    return blob


def loc(blob):
    if blob is not None:
        m = cv2.moments(blob)
        cx = int(m["m10"] / m["m00"])
        cy = int(m["m01"] / m["m00"])
        return (
            atan2(cy - h, cx - w) / pi * 180,
            sqrt(pow(cy - h, 2) + pow(cx - w, 2)),
            cx,
            cy,
        )  # angle, distance, w, h
    else:
        return None


def draw(image, blob, color=(0, 0, 255)):
    if blob is not None:
        angle, distance, bx, by = loc(blob)
        cv2.line(
            image,
            (mw, mh),
            (int(bx), int(by)),
            color,
        )
        cv2.drawContours(image, [blob], 0, (0, 255, 0), 1)


def preprocess(image):
    fr = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)
    ke = adjust_gamma(fr, 0.7)
    return ke

def rgbhsv(c1, c2, c3):
    return cv2.cvtColor(np.uint8([[[c1, c2, c3]]]), cv2.COLOR_RGB2HSV)[0][0]
