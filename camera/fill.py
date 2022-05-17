from time import sleep, time

import cv2
import numpy as np

from handlers.utils import *
from handlers.constants import *

im = cv2.imread("fshot.png")
cv2.imshow("nya", im)
cv2.waitKey(0)
cv2.destroyAllWindows()
