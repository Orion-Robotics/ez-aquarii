#include <Adafruit_BNO055.h>
#include <Adafruit_MCP3008.h>
#include <Arduino.h>
#include <Wire.h>

#include <array>

#include "SerialReader.h"

auto adcs = std::array<Adafruit_MCP3008, LINE_ADC_PINS.size()>();
auto input = SerialReader(CONTROLLER_PORT);
auto bno = Adafruit_BNO055(55);
int last_received = 0;

void applyCommands() {
  input.update();
  if ((millis() - last_received) > TIMEOUT) {
    for (auto& pin : DIR_PINS) {
      analogWrite(pin, 0);
    }
    for (auto& pin : MOVE_PINS) {
      analogWrite(pin, 0);
    }
  }
  if (!input.complete()) {
    return;
  }
  last_received = millis();
  const auto data = input.data();

  for (int i = 0; i < data.size(); i++) {
    if (i > 3) {
      Serial.printf("<!> extra motor value, %d\r\n", data[i]);
      continue;
    }
    const auto value = data[i];
    const auto forward = value > 127 ? true : false;
    const auto speed_command = abs(value - 127);
    analogWrite(MOVE_PINS[i], speed_command);
    analogWrite(DIR_PINS[i], forward ? 255 : 0);
  }
}

void setup() {
  CONTROLLER_PORT.begin(CONTROLLER_BAUD);
  Serial.begin(9600);
  while (!CONTROLLER_PORT) continue;

  if (!bno.begin()) {
    Serial.println("<!> BNO055 not found");
    while (1) continue;
  }
  bno.setExtCrystalUse(true);
  uint8_t system, gyro, accel, mag;
  while (mag != 3) {
    Serial.printf("It's not fully calibrated : %d, %d, %d %d\r\n", system, gyro, accel, mag);
    bno.getCalibration(&system, &gyro, &accel, &mag);
    delay(100);
  }

  for (int i = 0; i < LINE_ADC_PINS.size(); i++) {
    // (sck, mosi, miso, cs);
    const auto cs = LINE_ADC_PINS[i];
    Serial.println(i);
    adcs[i].begin(LINE_SCK, LINE_MOSI, LINE_MISO, cs);
  }

  for (auto pin : MOVE_PINS) {
    analogWriteFrequency(pin, 19500);  // THE ONE TRUE FREQUENCY
                                       // TO ACHIEVE INNER HARMONY
                                       // WITH THE UNIVERSE
                                       // anything between
                                       // 20000 and 19000 seems to work well
  }
}

int last_freq_update = millis();

void loop() {
  applyCommands();
  sensors_event_t ev;
  bno.getEvent(&ev);
  auto rotation = ev.orientation.roll * (M_PI / 180);
  CONTROLLER_PORT.print(rotation, 2);
  CONTROLLER_PORT.print(" ");
  for (auto i = 0; i < adcs.size(); i++) {
    for (auto channel = 0; channel < 8; channel++) {
      const auto channel_num = (i * 8) + channel;
      if (channel_num == 32 || channel_num == 33) continue;
      const auto value = adcs[i].readADC(7 - channel);
      const auto magnitude = (uint8_t)((value / 2048.0) * 255);
      Serial.printf("%d ", magnitude);
      CONTROLLER_PORT.printf("%d ", magnitude);
    }
  }
  Serial.println();
  CONTROLLER_PORT.println();
}
