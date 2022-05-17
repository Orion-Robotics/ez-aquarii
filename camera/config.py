import json
from typing import Callable


class Config:
    def __init__(self, path: str):
        self.schema = self.default_schema()
        self.path = path
        self.listeners = []
        self.page: int | None = None
        try:
            with open(path, "r+") as f:
                schema = json.load(f)
                self.schema = schema
        except:
            pass

    def default_schema(self):
        return {
            "thresholds": [
                [255, 0, 255, 0, 255, 0],
                [255, 0, 255, 0, 255, 0],
                [255, 0, 255, 0, 255, 0],
            ],
            "camera": {
                "saturation": 0,
            },
            "bypass": False,
        }

    def publish(self):
        for listener in self.listeners:
            listener(self)

    def update(self):
        json.dump(
            self.schema,
            open(self.path, "w+"),
        )
        self.publish()

    def add_listener(self, listener):
        self.listeners.append(listener)
