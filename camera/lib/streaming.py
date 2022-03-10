import io
import logging
import socketserver
import threading
from http import server
from threading import Condition

import cv2
import numpy as np
from handlers import BaseFrameHandler, constants

PAGE = f"""\
<html>
<head>
<title>Raspberry Pi - Surveillance Camera</title>
</head>
<body>
<center><h1>Raspberry Pi - Surveillance Camera</h1></center>
<center><img src="stream.mjpg" width="{constants.w}" height="{constants.h}"></center>
</body>
</html>
"""


class StreamingOutput(object):
    def __init__(self):
        self.frame = None
        self.buffer = io.BytesIO()
        self.condition = Condition()

    def write(self, buf):
        if buf.startswith(b"\xff\xd8"):
            # New frame, copy the existing buffer's content and notify all
            # clients it's available
            self.buffer.truncate()
            with self.condition:
                self.frame = self.buffer.getvalue()
                self.condition.notify_all()
            self.buffer.seek(0)
        return self.buffer.write(buf)


def generate_stream(output: StreamingOutput):
    class StreamingHandler(server.BaseHTTPRequestHandler):
        def do_GET(self):
            self.send_response(200)
            self.send_header("Age", str(0))
            self.send_header("Cache-Control", "no-cache, private")
            self.send_header("Pragma", "no-cache")
            self.send_header(
                "Content-Type", "multipart/x-mixed-replace; boundary=FRAME"
            )
            self.end_headers()
            try:
                while True:
                    with output.condition:
                        output.condition.wait()
                        frame = output.frame
                    if not frame:
                        continue
                    self.wfile.write(b"--FRAME\r\n")
                    self.send_header("Content-Type", "image/jpeg")
                    self.send_header("Content-Length", str(len(frame)))
                    self.end_headers()
                    self.wfile.write(frame)
                    self.wfile.write(b"\r\n")
            except Exception as e:
                logging.warning(
                    "Removed streaming client %s: %s", self.client_address, str(e)
                )

    return StreamingHandler


class StreamingServer(socketserver.ThreadingMixIn, server.HTTPServer):
    allow_reuse_address = True
    daemon_threads = True


class StreamingFrameHandler(BaseFrameHandler):
    def __init__(self, inner: BaseFrameHandler, addr: tuple[str, int]) -> None:
        super().__init__()
        self.inner = inner

        self.output = StreamingOutput()

        self.server = StreamingServer(addr, generate_stream(self.output))
        threading.Thread(target=self.server.serve_forever).start()

    def stop(self):
        self.server.shutdown()
        self.server.server_close()

    def handle_frame(self, frame: np.ndarray) -> np.ndarray:
        res = self.inner.handle_frame(frame)
        _, encoded = cv2.imencode(".jpg", res)
        self.output.write(encoded.tobytes())
        return res
