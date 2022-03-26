#include <Arduino.h>

class SerialReader {
 private:
  usb_serial_class input;
  bool _complete = false;
  String line;
  String buffer;

 public:
  SerialReader(usb_serial_class input) {
    this->input = input;
    this->_complete = false;
    this->line = String();
    this->buffer = String();
  }

  void update() {
    const auto count = input.available();
    if (count > 0) {
      for (auto i = 0; i < count; i++) {
        const char c = input.read();
        if (c == '\n') {
          this->line = String(this->buffer);
          this->buffer = "";
          _complete = true;
        } else {
          this->buffer.concat(c);
        }
      }
    }
  }

  const String data() {
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
