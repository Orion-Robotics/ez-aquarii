from math import atan2, pi, pow, sqrt
from typing import Any, Callable, Tuple

import cv2
import numpy as np

from handlers.math_ext import similarity

from .constants import *


class clockwise_angle_and_distance:
    """
    A class to tell if point is clockwise from origin or not.
    This helps if one wants to use sorted() on a list of points.

    Parameters
    ----------
    point : ndarray or list, like [x, y]. The point "to where" we g0
    self.origin : ndarray or list, like [x, y]. The center around which we go
    refvec : ndarray or list, like [x, y]. The direction of reference

    use:
        instantiate with an origin, then call the instance during sort
    reference:
    https://stackoverflow.com/questions/41855695/sorting-list-of-two-dimensional-coordinates-by-clockwise-angle-using-python

    Returns
    -------
    angle

    distance


    """

    def __init__(self, origin):
        self.origin = origin

    def __call__(self, point, refvec=[0, 1]):
        if self.origin is None:
            raise NameError("clockwise sorting needs an origin. Please set origin.")
        # Vector between point and the origin: v = p - o
        vector = [point[0] - self.origin[0], point[1] - self.origin[1]]
        # Length of vector: ||v||
        lenvector = np.linalg.norm(vector[0] - vector[1])
        # If length is zero there is no angle
        if lenvector == 0:
            return -pi, 0
        # Normalize vector: v/||v||
        normalized = [vector[0] / lenvector, vector[1] / lenvector]
        dotprod = normalized[0] * refvec[0] + normalized[1] * refvec[1]  # x1*x2 + y1*y2
        diffprod = (
            refvec[1] * normalized[0] - refvec[0] * normalized[1]
        )  # x1*y2 - y1*x2
        angle = atan2(diffprod, dotprod)
        # Negative angles represent counter-clockwise angles so we need to
        # subtract them from 2*pi (360 degrees)
        if angle < 0:
            return 2 * pi + angle, lenvector
        # I return first the angle because that's the primary sorting criterium
        # but if two vectors have the same angle then the shorter distance
        # should come first.
        return angle, lenvector


def adjust_gamma(image, gamma=1.0):
    # build a lookup table mapping the pixel values [0, 255] to
    # their adjusted gamma values
    invGamma = 1.0 / gamma
    table = np.array(
        [((i / 255.0) ** invGamma) * 255 for i in np.arange(0, 256)]
    ).astype("uint8")
    # apply gamma correction using the lookup table
    return cv2.LUT(image, table)


def crop_surroundings(im: np.ndarray):
    bl = np.zeros(im.shape[:2], dtype="uint8")
    bl = cv2.circle(bl, (mw, mh), mw, 255, -1)
    cr = cv2.bitwise_and(im, im, mask=bl)
    return cr


def draws(im: np.ndarray, target):
    bl = np.zeros(im.shape[:2], dtype="uint8")
    upper = np.array([target[0], target[2], target[4]])
    lower = np.array([target[1], target[3], target[5]])
    mask = cv2.inRange(im, lower, upper)
    contours, _ = cv2.findContours(mask, cv2.RETR_TREE, cv2.CHAIN_APPROX_NONE)
    cv2.drawContours(
        im,
        [contour for contour in contours if cv2.contourArea(contour) > 1000],
        -1,
        (255, 255, 255),
        3,
    )


def fill(im: np.ndarray, target):
    bl = np.zeros(im.shape[:2], dtype="uint8")
    upper = np.array([target[0], target[2], target[4]])
    lower = np.array([target[1], target[3], target[5]])
    mask = cv2.inRange(im, lower, upper)
    contours, _ = cv2.findContours(mask, cv2.RETR_TREE, cv2.CHAIN_APPROX_NONE)
    origin = np.array(list_of_pts).mean(axis=0)  # get origin
    clock_ang_dist = clockwise_angle_and_distance(origin)  # set origin
    list_of_pts = []
    for ctr in contours:
        list_of_pts += [pt[0] for pt in ctr]
    list_of_pts = sorted(list_of_pts, key=clock_ang_dist)  # use to sort
    ctr = np.array(list_of_pts).reshape((-1, 1, 2)).astype(np.int32)
    ctr = cv2.convexHull(ctr)  # done.
    cv2.drawContours(bl, [ctr], 0, (255, 255, 255), cv2.FILLED)
    cr = cv2.bitwise_and(im, im, mask=bl)
    return cr


def mask(image, target, blur=False, erode=False):
    upper = np.array([target[0], target[2], target[4]])
    lower = np.array([target[1], target[3], target[5]])
    # lower = np.absolute(np.array([target[0] - HDIFF, target[1] - SDIFF, target[2] - VDIFF]))
    # upper = np.absolute(np.array([target[0] + HDIFF, target[1] + SDIFF, target[2] + VDIFF]))
    mask = cv2.inRange(image, lower, upper)
    if blur:
        mask = cv2.GaussianBlur(mask, (5, 5), 0)
    if erode:
        mask = ed(mask, size=101)
    return cv2.bitwise_and(image, image, mask=mask)


def contour_squareness(contour):
    (_, (w, h), _) = cv2.minAreaRect(contour)
    return similarity(w, h)


HeuristicFunc = Callable[[Any], float]

# function that returns a "ball" heuristic, how similar a contour is to a ball
def ball_heuristic(
    contour, roundness_influence=1.0, area_influence=1.0, squareness_influence=1.0
):
    perimeter: float = cv2.arcLength(contour, True)
    area: float = area_influence * cv2.contourArea(contour)
    if perimeter == 0:  # perimeter is 0, this cant be a ball
        return 0
    roundness = roundness_influence * ((4 * pi * area) / pow(perimeter, 2))
    squareness = squareness_influence * contour_squareness(contour)
    return area * roundness * squareness


def find_optimal_blob(image: np.ndarray, target, heuristic: HeuristicFunc, min_size=10):
    upper = np.array([target[0], target[2], target[4]])
    lower = np.array([target[1], target[3], target[5]])
    # lower = np.absolute(np.array([target[0] - HDIFF, target[1] - SDIFF, target[2] - VDIFF]))
    # upper = np.absolute(np.array([target[0] + HDIFF, target[1] + SDIFF, target[2] + VDIFF]))
    mask = cv2.inRange(image, lower, upper)
    # mask = cv2.GaussianBlur(mask, (5, 5), 0)
    contours, _ = cv2.findContours(mask, cv2.RETR_TREE, cv2.CHAIN_APPROX_NONE)
    contours = filter(lambda contour: cv2.contourArea(contour) > min_size, contours)
    try:
        blob = max(contours, key=lambda el: cv2.contourArea(el))
        # for cont in contours:
        #     if cv2.contourArea(cont) > 200:
        #         print(cv2.contourArea(cont))
    except:
        return None
    return blob


def ed(im: np.ndarray, size=5):
    e = cv2.erode(im, (size, size), iterations=3)
    d = cv2.dilate(e, (size, size), iterations=3)
    return d


def exists(blob):
    return blob is not None and cv2.moments(blob)["m00"] != 0
    # if len(contours) == 0:
    #     return None
    # return max(contours, key=lambda el: heuristic(el))


# angle, distance, x, y
def loc(blob, center=(mw, mh)):
    m = cv2.moments(blob)
    if m["m00"] != 0:
        cx = int(m["m10"] / m["m00"])
        cy = int(m["m01"] / m["m00"])
        return (
            atan2(cy - center[1], cx - center[0]),
            sqrt(pow(cy - center[1], 2) + pow(cx - center[0], 2)),
            cx,
            cy,
        )
    else:
        return None


def draw(image, blob, center: Tuple[int, int], color=(255, 255, 255)):
    result = loc(blob)
    if result is None:
        return
    _, _, bx, by = result
    cv2.line(
        image,
        center,
        (int(bx), int(by)),
        color,
    )
    cv2.drawContours(image, [blob], 0, (255, 255, 255), 1)


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
