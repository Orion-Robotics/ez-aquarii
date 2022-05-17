import json

import cv2
import msgpack
import numpy as np
from config import Config
from lib.ipc import IPC

from . import BaseFrameHandler
from .constants import *
from .utils import *


class DisplayHandler(BaseFrameHandler):
    def __init__(self, ipc: IPC | None, enable_window: bool) -> None:
        super().__init__()
        self.ipc = ipc
        self.enable_window = enable_window
        self.ball_thresholds = [255, 0, 255, 0, 255, 0]
        self.yellow_goal_thresholds = [255, 0, 255, 0, 255, 0]
        self.blue_goal_thresholds = [255, 0, 255, 0, 255, 0]

        if enable_window:
            cv2.namedWindow("meow", cv2.WINDOW_NORMAL)

    def handle_config_update(self, config: Config) -> None:
        self.ball_thresholds = config.schema["thresholds"][0]
        self.yellow_goal_thresholds = config.schema["thresholds"][1]
        self.blue_goal_thresholds = config.schema["thresholds"][2]

    def handle_frame(self, frame: np.ndarray) -> np.ndarray:
        im = cv2.cvtColor(frame, cv2.COLOR_BGR2HSV)
        im = mask(crop_surroundings(im), self.ball_thresholds)
        blob = find_optimal_blob(im, self.ball_thresholds, ball_heuristic)
        if blob is not None:
            draw(im, blob)
            location = loc(blob, center=(mw, mh))
            if location is not None and self.ipc is not None:
                (angle, distance, _, _) = location
                self.ipc.send_data(
                    msgpack.packb(
                        {
                            "angle": angle,
                            "distance": distance,
                        }
                    )
                )
        im = cv2.cvtColor(im, cv2.COLOR_HSV2RGB)
        if self.enable_window:
            cv2.imshow("meow", im)
            cv2.waitKey(1)
        return im
