import cv2
import numpy as np

from . import BaseFrameHandler
from .constants import *
from .utils import *


class TestHandler(BaseFrameHandler):
    def __init__(self) -> None:
        super().__init__()

    def handle_frame(self, frame: np.ndarray):
        pass
