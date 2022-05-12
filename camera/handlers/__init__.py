import sys

import numpy as np


# do not implement
# instead, extend this class and pass that in instead
class BaseFrameHandler:
    def handle_frame(self, frame: np.ndarray) -> np.ndarray:
        sys.exit("dumbass")
        pass

    def handle_request(self, path: str, body: bytes) -> None:
        pass
