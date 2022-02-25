import numpy as np
import cv2

HDIFF = 20
SDIFF = 40
VDIFF = 60

frame = cv2.imread("image.jpg")
hsv = frame
lower_blue = np.array([60, 35, 140])
upper_blue = np.array([180, 255, 255])
mask = cv2.inRange(hsv, lower_blue, upper_blue)
result = cv2.bitwise_and(frame, frame, mask=mask)
two = np.concatenate((result, frame), axis=1)


def click(event, x, y, flags, param):
    global mouseX, mouseY, two
    if event == cv2.EVENT_LBUTTONUP:
        col = two[y, x]
        lower_blue = np.array([col[0] - HDIFF, col[1] - SDIFF, col[2] - VDIFF])
        upper_blue = np.array([col[0] + HDIFF, col[1] + SDIFF, col[2] + VDIFF])
        mask = cv2.inRange(hsv, lower_blue, upper_blue)
        mask = cv2.GaussianBlur(mask, (5, 5), 0)
        result = cv2.bitwise_and(frame, frame, mask=mask)
        contours, _ = cv2.findContours(mask, cv2.RETR_TREE, cv2.CHAIN_APPROX_SIMPLE)
        m = 0
        for i in range(1, len(contours)):
            if len(contours[i]) > len(contours[m]):
                m = 1
        cv2.drawContours(result, contours, m, (0, 255, 0), 1)
        two = np.concatenate((result, frame), axis=1)
        cv2.imshow("frame", two)


cv2.namedWindow("frame")
cv2.setMouseCallback("frame", click)
cv2.imshow("frame", two)
cv2.waitKey(0)
cv2.destroyAllWindows()
