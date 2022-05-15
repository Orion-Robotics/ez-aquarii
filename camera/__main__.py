import json
import os
import traceback
from time import sleep, time

import cv2
import numpy as np

from config import Config
from handlers import BaseFrameHandler, constants
from handlers.display import DisplayHandler
from handlers.noop import NoopHandler
from handlers.test import TestHandler
from lib.camera import Camera
from lib.ipc import IPC, new_fifo_ipc
from lib.streaming import StreamingFrameHandler

if __name__ == "__main__":
    try:
        config = Config("camera.json")
        ipc = new_fifo_ipc("socket")
        handler = DisplayHandler(ipc, False)
        stream_handler = StreamingFrameHandler(
            handler,
            constants.SERVER_ADDRESS,
            [],
        )
        cam = Camera(stream_handler)

        def config_update_handler(path: str, body: bytes) -> bytes | None:
            if path == "/thresholds":
                schema = json.loads(body)
                if schema["reset"] == True:
                    config.thresholds = [255, 0, 255, 0, 255, 0]
                    config.saturation = 0
                else:
                    config.saturation = schema["saturation"]
                    config.thresholds = schema["thresholds"]
                config.update()
            if path == "/config":
                return json.dumps(config.serialize()).encode("utf-8")
            return None

        def handle_config_update(config: Config) -> None:
            cam.camera.saturation = config.saturation

        stream_handler.add_listener(config_update_handler)
        config.add_listener(handle_config_update)
        config.add_listener(handler.handle_config_update)
        config.update()
        cam.run()
        # joe = cv2.imread("cha.jpg")
        # joe = cv2.resize(joe, (600, 600))
        # while True:
        #    handler.handle_frame(joe)
    except Exception as e:
        print(e)
        traceback.print_exc()
        os._exit(0)
