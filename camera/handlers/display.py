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

Threshold = Tuple[int, int, int, int, int, int]


def process(
    im: np.ndarray, thresholds: Threshold, heuristic: HeuristicFunc
) -> Tuple[np.ndarray, Any | None]:
    im = mask(im, thresholds)
    blob = find_optimal_blob(im, thresholds, heuristic)
    return (im, blob)


def wrapped_process(tuple):
    return process(*tuple)


class DisplayHandler(BaseFrameHandler):
    def __init__(self, ipc: IPC | None, enable_window: bool) -> None:
        super().__init__()
        self.ipc = ipc
        self.pool = Pool(processes=16)
        self.enable_window = enable_window
        self.thresholds = [
            (255, 0, 255, 0, 255, 0),
            (255, 0, 255, 0, 255, 0),
            (255, 0, 255, 0, 255, 0),
        ]
        self.heuristics = [
            functools.partial(ball_heuristic, area_influence=0.4),
            functools.partial(ball_heuristic, area_influence=0.4),
            functools.partial(ball_heuristic, area_influence=0.4),
        ]
        self.page: int | None = None

        if enable_window:
            cv2.namedWindow("meow", cv2.WINDOW_NORMAL)

    def handle_config_update(self, config: Config) -> None:
        # 0 = ball
        # 1 = yellow
        # 2 = blue
        self.thresholds = config.schema["thresholds"]
        self.page = config.page

    def handle_frame(self, frame: np.ndarray) -> np.ndarray:
        im = crop_surroundings(cv2.cvtColor(frame, cv2.COLOR_BGR2HSV))

        if self.page is None:
            # process_results = self.pool.map(
            #     wrapped_process,
            #     [
            #         (im, self.thresholds[i], self.heuristics[i])
            #         for i in range(len(self.thresholds))
            #     ],
            # )
            process_results = [
                process(im, self.thresholds[i], self.heuristics[i])
                for i in range(len(self.thresholds))
            ]
        else:
            process_results = [
                process(im, self.thresholds[self.page], self.heuristics[self.page])
            ]

        im = process_results[0][0]
        for (masked, _) in process_results:
            im = cv2.bitwise_or(im, masked)

        for (_, blob) in process_results:
            if blob is not None:
                draw(im, blob)

        locations = [
            {
                "angle": location[0],
                "distance": location[1],
            }
            if location is not None
            else None
            for location in (
                loc(blob, center=(mw, mh)) for (_, blob) in process_results
            )
        ]
        if self.ipc is not None:
            self.ipc.send_data(msgpack.packb({"locations": locations}))
        im = cv2.cvtColor(im, cv2.COLOR_HSV2RGB)
        if self.enable_window:
            cv2.imshow("meow", im)
            cv2.waitKey(1)
        return im
