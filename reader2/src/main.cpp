#include <Adafruit_MCP3008.h>
#include <Arduino.h>
#include <Wire.h>

#include <array>

#include "SerialReader.h"

#define LINE_SCK 13
#define LINE_MOSI 11
#define LINE_MISO 12

#define CONTROLLER_PORT Serial
// DO NOT COMMENT OUT ADCS WITHOUT UNPLUGGING TEENSY
// HOURS WASTED: 1
const auto LINE_ADC_PINS = std::array<int, 6>{
    6,
    10,
    2,
    24,
    14,
    15,
};

auto adcs = std::array<Adafruit_MCP3008, LINE_ADC_PINS.size()>();
auto input = SerialReader(CONTROLLER_PORT);

// void applyCommands() {
//   input.update();
//   if (!input.complete()) return;
//   Serial.print("Received command: [");
//   for (auto value : input.data()) {
//     Serial.printf("%d, ", value);
//   }
//   Serial.print("]\r\n");
// }

void setup() {
  Serial.begin(9600);
  while (!Serial) {
    continue;
  }

  // pinMode(LINE_SCK, INPUT_PULLDOWN);
  // pinMode(LINE_MOSI, );

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
  // applyCommands();

  Serial.write(255);
  for (int i = 0; i < adcs.size(); i++) {
    for (int channel = 0; channel < 8; channel++) {
      const auto channel_num = (i * 8) + channel;
      if (channel_num == 32 || channel_num == 33) continue;
      const auto value = adcs[i].readADC(7 - channel);
      const auto magnitude = (uint8_t)((value / 2048.0) * 253);
      Serial.write(magnitude);
      // Serial.printf("%3d ", magnitude);
      // Serial.print(String(channel) + " " + String(i));
    }
  }
  // Serial.println();
}
