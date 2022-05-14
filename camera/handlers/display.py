import json

import cv2
import msgpack
import numpy as np
from lib.ipc import IPC

from . import BaseFrameHandler
from .constants import *
from .utils import *


class DisplayHandler(BaseFrameHandler):
    def __init__(self, ipc: IPC | None, enable_window: bool) -> None:
        super().__init__()
        try:
            with open("./camera.json", "r+") as f:
                self.thresholds = json.load(f)["thresholds"]
        except:
            self.thresholds = [255, 0, 255, 0, 255, 0]
        self.ipc = ipc
        self.enable_window = enable_window
        # ball, goal1, goal2
        self.current = 0

        if enable_window:
            cv2.namedWindow("meow", cv2.WINDOW_NORMAL)

    def handle_request(self, path: str, body: bytes) -> bytes | None:
        if path == "/get_thresholds":
            return json.dumps({"thresholds": self.thresholds}).encode("utf-8")
        if path == "/thresholds":
            self.thresholds = json.loads(body)["thresholds"]
            json.dump({"thresholds": self.thresholds}, open("./camera.json", "w"))
            return b"OK"
        return None

    def handle_frame(self, frame: np.ndarray) -> np.ndarray:
        im = cv2.cvtColor(frame, cv2.COLOR_BGR2HSV)
        bl = np.zeros(frame.shape[:2], dtype="uint8")
        bl = cv2.circle(bl, (mw, mh), mw, 255, -1)
        cr = cv2.bitwise_and(im, im, mask=bl)
        blob = find_blob(cr, self.thresholds)
        if blob is not None:
            draw(cr, blob)

        location = loc(blob, center=(mw, mh))
        if location is not None and self.ipc is not None:
            (angle, distance, x, y) = location
            self.ipc.send_data(
                msgpack.packb(
                    {
                        "angle": angle,
                        "distance": distance,
                    }
                )
            )
        im = mask(frame, self.thresholds)
        im = cv2.cvtColor(im, cv2.COLOR_BGR2RGB)
        if self.enable_window:
            cv2.imshow("meow", im)
            cv2.waitKey(1)
        return im
