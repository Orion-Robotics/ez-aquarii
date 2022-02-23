import numpy as np
import cv2

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
        lower_blue = np.array([col[0] - 40, col[1] - 40, col[2] - 40])
        upper_blue = np.array([col[0] + 40, col[1] + 40, col[2] + 40])
        mask = cv2.inRange(hsv, lower_blue, upper_blue)
        result = cv2.bitwise_and(frame, frame, mask=mask)
        two = np.concatenate((result, frame), axis=1)
        cv2.imshow("frame", two)


cv2.namedWindow("frame")
cv2.setMouseCallback("frame", click)
cv2.imshow("frame", two)
cv2.waitKey(0)
cv2.destroyAllWindows()
