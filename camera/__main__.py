import json
import os
import traceback
from time import sleep, time

import cv2
import numpy as np

from handlers import BaseFrameHandler, constants
from handlers.display import DisplayHandler
from handlers.noop import NoopHandler
from handlers.test import TestHandler
from lib.camera import Camera
from lib.ipc import IPC, new_fifo_ipc
from lib.streaming import StreamingFrameHandler

if __name__ == "__main__":
    try:
        ipc = new_fifo_ipc("socket")
        handler = DisplayHandler(ipc, False)
        handler = StreamingFrameHandler(
            handler,
            constants.SERVER_ADDRESS,
            [],
        )
        cam = Camera(handler)

        def wb_adjust(path: str, body: bytes) -> bytes | None:
            if path == "/wb":
                data = json.loads(body)
                print(data)
                cam.camera.awb_gains = (
                    data["red"],
                    data["blue"],
                )
                cam.camera.iso = data["iso"]
            return None

        handler.add_handler(wb_adjust)
        handler.add_handler(handler.handle_request)
        cam.run()
        # joe = cv2.imread("cha.jpg")
        # joe = cv2.resize(joe, (600, 600))
        # while True:
        #    handler.handle_frame(joe)
    except Exception as e:
        print(e)
        traceback.print_exc()
        os._exit(0)
