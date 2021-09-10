from os import mkfifo
import os
import unittest
from typing import BinaryIO

class IPC:
  def __init__(self, io: BinaryIO):
    self.io = io
  
  def send_data(self, data: bytes):
    size = len(data)
    self.io.write(bytes(size))
    self.io.write(data)
  
  def receive_data(self) -> bytes:
    size = int.from_bytes(self.io.read(2), byteorder='little')
    return self.io.read(size)

def new_fifo_ipc(path: str):
    mkfifo(path, 0o660)
    reader = open(path, 'wb')
    return IPC(reader)

class TestIPC(unittest.TestCase):
  def test_serialize(self):
    test_path = "camerastream"
    try:
      os.remove(test_path)
    except OSError:
      pass
    comms = new_fifo_ipc(test_path)
    comms.send_data(b"hello uwu")
    print("done")

if __name__ == "__main__":
  unittest.main()
