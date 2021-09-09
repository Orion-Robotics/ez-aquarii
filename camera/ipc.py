import sys
from typing import TextIO

class IPC:
  def __init__(self, io: TextIO):
    self.io = io
  
  def send_data(self, data: bytes):
    self.io.buffer.write(2.to_bytes())
