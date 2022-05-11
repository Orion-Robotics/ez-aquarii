import cv2
import imagezmq
import zmq

context = zmq.Context()

#  Socket to talk to server
print("Connecting to hello world server…")
socket = context.socket(zmq.REQ)
socket.connect("tcp://localhost:5555")

image_hub = imagezmq.ImageHub()
while True:  # show streamed images until Ctrl-C
    rpi_name, image = image_hub.recv_image()
    cv2.imshow(rpi_name, image)  # 1 window for each RPi
    cv2.waitKey(0)
    socket.send(b"AMOGUS")
