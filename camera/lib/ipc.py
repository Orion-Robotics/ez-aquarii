import os
import unittest
from os import mkfifo
from typing import BinaryIO

from genericpath import exists


class IPC:
    def __init__(self, io: BinaryIO):
        self.io = io

    def send_data(self, data: bytes):
        size = len(data)
        self.io.write(size.to_bytes(4, "little"))  # 4 bytes = 32 bits
        self.io.write(data)
        self.io.flush()

    def receive_data(self) -> bytes:
        size = int.from_bytes(self.io.read(2), byteorder="little")
        return self.io.read(size)


def new_fifo_ipc(path: str):
    if exists(path):
        os.remove(path)
    mkfifo(path, 0o660)
    writer = open(path, "rb+", 0)
    return IPC(writer)


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
