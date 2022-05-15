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
                self.reset = schema["reset"]
        except:
            self.thresholds = [255, 0, 255, 0, 255, 0]
            self.saturation = 0
            self.reset = False

    def serialize(self) -> dict:
        return {
            "thresholds": self.thresholds,
            "saturation": self.saturation,
            "reset": self.reset,
        }

    def publish(self):
        for listener in self.listeners:
            listener(self)

    def update(self):
        json.dump(
            self.serialize(),
            open(self.path, "w+"),
        )
        self.publish()

    def add_listener(self, listener):
        self.listeners.append(listener)
