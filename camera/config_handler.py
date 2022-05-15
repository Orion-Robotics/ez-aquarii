import json
from typing import Callable


class Config:
    def __init__(self, path: str):
        self.path = path
        self.listeners = []
        try:
            with open(path, "r+") as f:
                schema = json.load(f)
                self.thresholds = schema["thresholds"]
                self.saturation = schema["saturation"]
        except:
            self.thresholds = [255, 0, 255, 0, 255, 0]
            self.saturation = 0

    def serialize(self) -> dict:
        return {"thresholds": self.thresholds, "saturation": self.saturation}

    def update(self):
        json.dump(
            self.serialize(),
            open(self.path, "w+"),
        )
        for listener in self.listeners:
            listener(self)

    def add_listener(self, listener):
        self.listeners.append(listener)
