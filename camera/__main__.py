import json
import os
import traceback
from time import sleep, time

import cv2
import numpy as np

from config import Config
from handlers import constants
from handlers.display import DisplayHandler
from handlers.detection import DetectionHandler
from lib.camera import Camera
from lib.ipc import new_fifo_ipc
from lib.streaming import StreamingFrameHandler

if __name__ == "__main__":
    try:
        config = Config("camera.json")
        ipc = new_fifo_ipc("socket")
        handler = DetectionHandler(ipc, False)
        # handler = DisplayHandler()
        stream_handler = StreamingFrameHandler(
            handler,
            constants.SERVER_ADDRESS,
            [],
        )
        cam = Camera(stream_handler)

        def config_update_handler(path: str, body: bytes) -> bytes | None:
            if path == "/page":
                schema = json.loads(body)
                if schema["page"] == 0:
                    config.page = None
                else:
                    config.page = schema["page"] - 1
                config.publish()
            if path == "/config":
                schema = json.loads(body)
                if schema["bypass"] == True:
                    config.schema = config.default_schema()
                else:
                    config.schema = schema
                config.update()
            if path == "/get_config":
                return json.dumps(config.schema).encode("utf-8")
            return None

        def handle_config_update(config: Config) -> None:
            cam.camera.saturation = config.schema["camera"]["saturation"]

        stream_handler.add_listener(config_update_handler)
        config.add_listener(handle_config_update)
        config.add_listener(handler.handle_config_update)
        config.update()
        cam.run()
    except:
        traceback.print_exc()
        os._exit(0)
