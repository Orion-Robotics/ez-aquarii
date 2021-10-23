#include <Arduino.h>
#include <LTC2496.h>
#include <LT_SPI.h>

#define READER_CS 9
#define READER_CLOCK 13
#define READER_MOSI 11
#define READER_MISO 12
#define READER_VREF 3.3
#define MISO_TIMEOUT 1000

float readADC() {
  int adc_code = 0;

  LTC2496_read(READER_CS, LTC2496_CH0,
               &adc_code);  // Obtains the current reading and stores to
                            // adc_code variable

  // Convert adc_code to voltage
  float adc_voltage = LTC2496_code_to_voltage(adc_code, READER_VREF);
  return adc_voltage;
}

void setup() {
  pinMode(READER_CS, OUTPUT);
  digitalWrite(READER_CS, HIGH);
  pinMode(READER_CLOCK, OUTPUT);
  pinMode(READER_MOSI, OUTPUT);
  pinMode(READER_MISO, INPUT);

  quikeval_SPI_init();
  quikeval_SPI_connect();

  Serial.begin(9600);
  // put your setup code here, to run once:
  if (LTC2496_EOC_timeout(READER_CS, MISO_TIMEOUT))  // Check for EOC
    return 1;
  int adc_code = 0;
  LTC2496_read(READER_CS, LTC2496_CH0, &adc_code);   // Throws out last reading
  if (LTC2496_EOC_timeout(READER_CS, MISO_TIMEOUT))  // Check for EOC
    return 1;
}

int ticks = 0;
int lastMeasure = millis();
void loop() {
  // put your main code here, to run repeatedly:
  ticks++;
  Serial.println(readADC());
  if (millis() - lastMeasure > 1000) {
    //Serial.println(String(ticks) + " TPS");
    ticks = 0;
    lastMeasure = millis();
  }
  delay(100);
}
