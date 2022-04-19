#include <Adafruit_MCP3008.h>
#include <Arduino.h>
#include <Wire.h>

#include <array>

#include "SerialReader.h"

<<<<<<< HEAD
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
=======
auto adcs = std::array<Adafruit_MCP3008, LINE_ADC_PINS.size()>();
>>>>>>> e2d7a679e3834255243c881863e065a944846243
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
<<<<<<< HEAD
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
=======
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
>>>>>>> e2d7a679e3834255243c881863e065a944846243
}

void loop()
{
  //   applyCommands();

<<<<<<< HEAD
  for (size_t i = 0; i < LINE_ADC_PINS.size(); i++)
  {
    for (int channel = 0; channel < 8; channel++)
    {
      const int16_t value = adcs[i].readADC(channel);
      // Serial.print((uint8_t)((float)(value / 1024) * 254));
      Serial.printf("%3d", value);
=======
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
>>>>>>> e2d7a679e3834255243c881863e065a944846243
    }
    Serial.print("\t");
  }
<<<<<<< HEAD
  Serial.println();
  // Serial.println();
  // for (int i = 0; i < 4; i++)
  // {
  //   analogWrite(MOTOR_PINS[i].powerPin, 255);
  //   digitalWrite(MOTOR_PINS[i].directionPin, i < 2 ? 1 : 0);
  // }
  // Serial.println("amogsu");

  delay(10);
=======
  // Serial.println("meow");
  // Serial.println();
>>>>>>> e2d7a679e3834255243c881863e065a944846243
}
