# import the necessary packages
from picamera import PiCamera
from threading import Condition
import io
import time

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

with PiCamera(resolution='640x480', framerate=120) as camera:
    output = StreamingOutput()
    #Uncomment the next line to change your Pi's Camera rotation (in degrees)
    #camera.rotation = 90
    camera.start_recording(output, format='mjpeg')
    stamp = time.time()
    while True:
      with output.condition:
        output.condition.wait()
        frame = output.frame
        print(time.time() - stamp)
        stamp = time.time()
