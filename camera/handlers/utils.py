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
    frame = adjust_gamma(image, 0.8)
    hsv = cv2.cvtColor(frame, cv2.COLOR_BGR2HSV)
    return hsv


def detectlines(img):
    gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)

    kernel_size = 5
    blur_gray = cv2.GaussianBlur(gray, (kernel_size, kernel_size), 0)

    low_threshold = 50
    high_threshold = 150
    edges = cv2.Canny(blur_gray, low_threshold, high_threshold)

    rho = 1  # distance resolution in pixels of the Hough grid
    theta = np.pi / 180  # angular resolution in radians of the Hough grid
    threshold = 15  # minimum number of votes (intersections in Hough grid cell)
    min_line_length = 50  # minimum number of pixels making up a line
    max_line_gap = 20  # maximum gap in pixels between connectable line segments
    line_image = np.copy(img) * 0  # creating a blank to draw lines on

    # Run Hough on edge detected image
    # Output "lines" is an array containing endpoints of detected line segments
    lines = cv2.HoughLinesP(
        edges, rho, theta, threshold, np.array([]), min_line_length, max_line_gap
    )

    for line in lines:
        for x1, y1, x2, y2 in line:
            cv2.line(line_image, (x1, y1), (x2, y2), (255, 0, 0), 5)

    # Draw the lines on the  image
    lines_edges = cv2.addWeighted(line_image, 0.8, line_image, 1, 0)
    return line_image
