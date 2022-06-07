import functools
import json
from multiprocessing import Pool
from time import process_time
from typing import Tuple

import cv2
import msgpack
import numpy as np
from config import Config
from lib.ipc import IPC

from . import BaseFrameHandler
from .constants import *
from .utils import *

class DisplayHandler(BaseFrameHandler):
    def __init__(self) -> None:
        super().__init__()

    def handle_frame(self, frame: np.ndarray) -> np.ndarray:
        return frame
