#include <Adafruit_MCP3008.h>
#include <Arduino.h>
#include <Wire.h>

#include <array>

#include "SerialReader.h"

struct Motor
{
  uint8_t powerPin;
  uint8_t directionPin;
};

#define LINE_ADC_COUNT 6
#define LINE_SENSOR_COUNT 48
#define CONTROLLER_PORT Serial
const auto LINE_ADC_PINS = std::array<int, LINE_ADC_COUNT>{6, 2, 14, 17, 10, 15};
// { powerPin, directionPin }
const auto MOTOR_PINS = std::array<Motor, 4>{
    Motor{3, 4},
    Motor{9, 23},
    Motor{22, 20},
    Motor{17, 16},
};

auto adcs = std::array<Adafruit_MCP3008, 6>();
auto input = SerialReader(CONTROLLER_PORT);
int q = 0;

void setPower(Motor motor, short strength)
{
  analogWrite(motor.powerPin, strength);
  analogWrite(motor.directionPin, strength < 0);
}

void applyCommands()
{
  input.update();
  if (!input.complete())
    return;

  const auto data = input.data().c_str();
  for (auto i = 0; i < MOTOR_PINS.size(); i++)
  {
    setPower(MOTOR_PINS[i], (short)data[i]);
  }
}

void setup()
{
  // Serial.begin(500000);
  // while (!Serial)
  // {
  //   continue;
  // }

  // for (int i = 0; i < LINE_ADC_PINS.size(); i++)
  // {
  //   // (sck, mosi, miso, cs);
  //   adcs[i].begin(13, 11, 12, LINE_ADC_PINS[i]);
  // }
  // for (auto m : MOTOR_PINS)
  // {
  //   pinMode(m.directionPin, OUTPUT);
  //   pinMode(m.powerPin, OUTPUT);
  // }

  // bool controllerStarted;
  // while (!controllerStarted)
  // {
  //   Serial.write(255);
  //   if (Serial.read() == 255)
  //   {
  //     controllerStarted = true;
  //   }
  // }
}

void loop()
{
  //   applyCommands();

  for (size_t i = 0; i < LINE_ADC_PINS.size(); i++)
  {
    for (int channel = 0; channel < 8; channel++)
    {
      const int16_t value = adcs[i].readADC(channel);
      // Serial.print((uint8_t)((float)(value / 1024) * 254));
      Serial.printf("%3d", value);
    }
    Serial.print("\t");
  }
  Serial.println();
  // Serial.println();
  // for (int i = 0; i < 4; i++)
  // {
  //   analogWrite(MOTOR_PINS[i].powerPin, 255);
  //   digitalWrite(MOTOR_PINS[i].directionPin, i < 2 ? 1 : 0);
  // }
  // Serial.println("amogsu");

  delay(10);
}
