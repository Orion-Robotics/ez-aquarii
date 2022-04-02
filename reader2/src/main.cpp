#include <Adafruit_MCP3008.h>
#include <Arduino.h>
#include <Wire.h>

#include <array>

#include "SerialReader.h"

#define LINE_ADC_COUNT 6
#define LINE_SENSOR_COUNT 48
#define CONTROLLER_PORT Serial
const auto LINE_ADC_PINS = std::array<int, LINE_ADC_COUNT>{6, 2, 14, 17, 10, 15};

auto adcs = std::array<Adafruit_MCP3008, 6>();
auto input = SerialReader(CONTROLLER_PORT);

void applyCommands() {
  input.update();
  if (!input.complete()) return;
  Serial.print("Received command: [");
  for (auto value : input.data()) {
    Serial.printf("%d, ", value);
  }
  Serial.print("]\r\n");
}

void setup() {
  Serial.begin(9600);
  while (!Serial) {
    continue;
  }

  for (int i = 0; i < LINE_ADC_PINS.size(); i++) {
    // (sck, mosi, miso, cs);
    adcs[i].begin(13, 11, 12, LINE_ADC_PINS[i]);
  }
}

void loop() {
  applyCommands();

  // Serial.write(255);
  for (size_t i = 0; i < LINE_ADC_PINS.size(); i++) {
    for (int channel = 0; channel < 8; channel++) {
      // const auto channel_num = (i * 8) + channel;
      // if (channel_num == 22 || channel_num == 23) continue;
      // const auto value = adcs[i].readADC(channel);
      // const auto magnitude = value / 2048.0;
      // Serial.write((uint8_t)(magnitude * 253));
      Serial.printf("%d, ", adcs[i].readADC(channel));
    }
  }
  Serial.println();
}
