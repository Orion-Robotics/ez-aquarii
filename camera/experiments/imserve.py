# run this program on each RPi to send a labelled image stream
import socket
import time
from imutils.video import VideoStream
import imagezmq
import cv2
import zmq

context = zmq.Context()
zmq.RCVTIMEO = 1000
sock = context.socket(zmq.REP)
sock.bind("tcp://*:7777")

a = cv2.imread("a.png")
sender = imagezmq.ImageSender(connect_to="tcp://localhost:5555")
rpi_name = socket.gethostname()  # send RPi hostname with each image
# picam = VideoStream(usePiCamera=True).start()

time.sleep(2.0)  # allow camera sensor to warm up
while True:  # send images as stream until Ctrl-C
    # image = picam.read()
    try:
        sender.send_image(
            rpi_name,
            a,
        )
        message = sock.recv()
        print(message)
    except:
        exit()
