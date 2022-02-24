import numpy as np
import cv2

HDIFF = 35
SDIFF = 35
VDIFF = 80
# params = cv2.SimpleBlobDetector_Params()
# params.minThreshold = 10
# params.maxThreshold = 200
# params.filterByArea = False
# params.filterByCircularity = True
# params.minCircularity = 0.3
# params.filterByInertia = True
# params.minInertiaRatio = 0.01

# detector = cv2.SimpleBlobDetector_create(params)
frame = cv2.imread("image.jpg")
hsv = frame  # cv2.cvtColor(frame, cv2.COLOR_BGR2HSV)
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
        cv2.drawContours(result, contours, -1, (0, 255, 0), 1)
        two = np.concatenate((result, frame), axis=1)
        cv2.imshow("frame", two)


cv2.namedWindow("frame")
cv2.setMouseCallback("frame", click)
cv2.imshow("frame", two)
cv2.waitKey(0)
cv2.destroyAllWindows()
