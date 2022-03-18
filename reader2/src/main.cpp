/***************************************************
Simple example of reading the MCP3008 analog input channels and printing
them all out.

Author: Carter Nelson
License: Public Domain
****************************************************/

#include <Adafruit_MCP3008.h>
#include <Wire.h>
#include <Arduino.h>

// Adafruit_MCP3008 adc;
Adafruit_MCP3008 adcs[6];

int count = 0;
int pins[6] = {6, 2, 14, 17, 10, 15};
int vals[48] = {0};

void setup()
{
  Serial.begin(9600);
  while (!Serial)
    ;

  for (int i = 0; i < 6; i++)
  {
    adcs[i].begin(13, 11, 12, pins[i]);
  }

  // (sck, mosi, miso, cs);
  // adc.begin(13, 11, 12, 10); // 6 2 14 17 10 15
}

void loop()
{
  int q;
  for (int adc = 0; adc < 6; adc++)
  {
    for (int channel = 0; channel < 8; channel++)
    {
      q = adcs[t].readADC(channel);
      Serial.print(String(q) + " ");
      vals[6 * adc + channel] = q;
    }
    Serial.print("\t");
  }
  Serial.println();
  delay(200);
}