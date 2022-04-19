#include <Adafruit_MCP3008.h>
#include <Arduino.h>
#include <Wire.h>

#include <array>

#include "SerialReader.h"

auto adcs = std::array<Adafruit_MCP3008, LINE_ADC_PINS.size()>();
auto input = SerialReader(CONTROLLER_PORT);

void applyCommands() {
  input.update();
  if (!input.complete()) return;
  const auto data = input.data();
  for (int i = 0; i < data.size(); i++) {
    if (i > 3) {
      Serial.printf("<!> extra motor value, %d\r\n", data[i]);
      continue;
    }
    const auto value = data[i];
    const auto forward = value > 127 ? true : false;
    const auto speed = abs((value - 127) * 2);
    analogWrite(MOVE_PINS[i], (speed / 253.0) * SPEED);
    analogWrite(DIR_PINS[i], forward ? 255 : 0);
  }
  Serial.println();
}

void setup() {
  CONTROLLER_PORT.begin(CONTROLLER_BAUD);
  Serial.begin(9600);
  while (!CONTROLLER_PORT) continue;
  // pinMode(LINE_SCK, INPUT_PULLDOWN);
  // pinMode(LINE_MOSI, );
  // input.sync();

  for (int i = 0; i < LINE_ADC_PINS.size(); i++) {
    // (sck, mosi, miso, cs);
    const auto cs = LINE_ADC_PINS[i];
    Serial.println(i);
    // pinMode(cs, OUTPUT);
    // digitalWrite(cs, HIGH);
    adcs[i].begin(LINE_SCK, LINE_MOSI, LINE_MISO, cs);
  }
}

void loop() {
  applyCommands();

  CONTROLLER_PORT.write(255);
  for (int i = 0; i < adcs.size(); i++) {
    for (int channel = 0; channel < 8; channel++) {
      const auto channel_num = (i * 8) + channel;
      if (channel_num == 32 || channel_num == 33) continue;
      const auto value = adcs[i].readADC(7 - channel);
      const auto magnitude = (uint8_t)((value / 2048.0) * 253);
      CONTROLLER_PORT.write(magnitude);
      // Serial.printf("%3d ", magnitude);
      // Serial.print(String(channel) + " " + String(i));
    }
  }
  // Serial.println("meow");
  // Serial.println();
}
