#include <Arduino.h>

#include <vector>

#include "config.h"

class SerialReader {
 private:
  bool _complete = false;
  std::vector<uint8_t> line;
  std::vector<uint8_t> buffer;

 public:
  SerialReader(int baud = 115200) {
    this->_complete = false;
    this->line = std::vector<uint8_t>();
    this->buffer = std::vector<uint8_t>();
  }

  void sync() {
    while (CONTROLLER_PORT.read() != 255) continue;
  }

  void update() {
    const auto count = CONTROLLER_PORT.available();
    if (count > 0) {
      for (auto i = 0; i < count; i++) {
        const uint8_t c = CONTROLLER_PORT.read();
        if (c == 255) {
          this->line = std::vector<uint8_t>(this->buffer);
          this->buffer = std::vector<uint8_t>();
          _complete = true;
        } else {
          this->buffer.push_back(c);
        }
      }
    }
  }

  const std::vector<uint8_t> data() {
    return line;
  }

  bool complete() {
    if (_complete) {
      _complete = false;
      return true;
    }
    return false;
  }
};
