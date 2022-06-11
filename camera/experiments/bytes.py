import io
import time
import picamera
import cv2 
import numpy as np

cap = cv2.VideoCapture(0)

while(True):
    ret, frame = cap.read()
    cv2.imshow('gr', frame)
    if cv2.waitKey(1) & 0xFF == ord('q'):
        break
