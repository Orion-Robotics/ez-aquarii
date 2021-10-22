#include <Arduino.h>
#include <LTC2496/LTC2496.h>

#define READER_CS 9

void setup()
{
  // put your setup code here, to run once:
}

void loop()
{
  // put your main code here, to run repeatedly:
  uint16_t miso_timeout = 1000;
  int32_t adc_code = 0;
  if (LTC2496_EOC_timeout(READER_CS, miso_timeout)) // Check for EOC
    return (1);
  LTC2496_read(READER_CS, LTC2496_CH0, &adc_code); // Throws out last reading

  if (LTC2496_EOC_timeout(READER_CS, miso_timeout)) // Check for EOC
    return (1);
  LTC2496_read(READER_CS, LTC2496_CH0, &adc_code); // Obtains the current reading and stores to adc_code variable

  // Convert adc_code to voltage
  adc_voltage = LTC2496_code_to_voltage(adc_code, LTC2496_lsb, LTC2496_offset_code);
}