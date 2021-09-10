# import the necessary packages
from lib.ipc import new_fifo_ipc
from picamera import PiCamera
from threading import Condition
import io
from gen.protocol.comms_pb2 import Packet
from os import mkfifo
import os

class StreamingOutput(object):
    def __init__(self):
        self.frame = None
        self.buffer = io.BytesIO()
        self.condition = Condition()

    def write(self, buf):
        if buf.startswith(b'\xff\xd8'):
            # New frame, copy the existing buffer's content and notify all
            # clients it's available
            self.buffer.truncate()
            with self.condition:
                self.frame = self.buffer.getvalue()
                self.condition.notify_all()
            self.buffer.seek(0)
        return self.buffer.write(buf)

STREAM_PATH = "camerastream"

try:
  os.remove(STREAM_PATH)
except OSError:
  pass

comms = new_fifo_ipc(STREAM_PATH)

with PiCamera(resolution='640x480', framerate=120) as camera:
    output = StreamingOutput()
    #Uncomment the next line to change your Pi's Camera rotation (in degrees)
    #camera.rotation = 90
    camera.start_recording(output, format='mjpeg')
    while True:
      with output.condition:
        output.condition.wait()
        frame = output.frame
        out = Packet()
        out.time.GetCurrentTime()
        comms.send_data(out.SerializeToString())
