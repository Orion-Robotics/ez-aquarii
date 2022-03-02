import numpy as np
import cv2
from math import atan2, pow, sqrt, pi

HDIFF = 15
SDIFF = 30
VDIFF = 30

frame = cv2.imread("image.jpg")
hsv = cv2.cvtColor(frame, cv2.COLOR_BGR2HSV)
# hsv = cv2.GaussianBlur(hsv, (9, 9), 0)
lower_blue = np.array([60, 35, 140])
upper_blue = np.array([180, 255, 255])
mask = cv2.inRange(hsv, lower_blue, upper_blue)
result = cv2.bitwise_and(frame, frame, mask=mask)
two = np.concatenate((result, frame), axis=1)


def click(event, x, y, flags, param):
    global mouseX, mouseY, two
    if event == cv2.EVENT_LBUTTONUP:
        col = hsv[y, x]
        lower = np.array([col[0] - HDIFF, col[1] - SDIFF, col[2] - VDIFF])
        upper = np.array([col[0] + HDIFF, col[1] + SDIFF, col[2] + VDIFF])
        mask = cv2.inRange(hsv, lower, upper)
        mask = cv2.GaussianBlur(mask, (5, 5), 0)
        result = cv2.bitwise_and(frame, frame, mask=mask)
        contours, _ = cv2.findContours(mask, cv2.RETR_TREE, cv2.CHAIN_APPROX_NONE)
        # m = 0
        # for i in range(1, len(contours)):
        #     if len(contours[i]) > len(contours[m]):
        #         m = 1
        blob = max(contours, key=lambda el: cv2.contourArea(el))
        cv2.drawContours(result, [blob], 0, (0, 255, 0), 1)
        m = cv2.moments(blob)
        cx = int(m["m10"] / m["m00"])
        cy = int(m["m01"] / m["m00"])
        w = int(mask.shape[0] / 2)
        h = int(mask.shape[1] / 2)
        print(atan2(cy - h, cx - w) / pi * 180)
        cv2.line(
            result,
            (350, 200),
            (int(cx), int(cy)),
            (0, 0, 255),
        )
        print(sqrt(pow(cy - h, 2) + pow(cx - w, 2)))
        two = np.concatenate(
            (
                result,
                frame,
            ),
            axis=1,
        )
        cv2.imshow("frame", frame)
        cv2.imshow("result", result)


cv2.namedWindow("frame")
cv2.namedWindow("result")
cv2.setMouseCallback("frame", click)
cv2.imshow("frame", frame)
cv2.imshow("result", result)
cv2.waitKey(0)
cv2.destroyAllWindows()
