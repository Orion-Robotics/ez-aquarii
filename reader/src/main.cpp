#include <Arduino.h>
#include <LTC2496.h>
#include <LTC24XX_general.h>
#include <LT_SPI.h>

#define LINE_SENSOR_COUNT 46
#define READER_CS 9
#define READER0_CS 10
#define READER1_CS 6
#define READER2_CS 17
#define READER_CLOCK 13
#define READER_MOSI 11
#define READER_MISO 12
#define READER_VREF 3.3
#define MISO_TIMEOUT 2147483647

int gochans[16] = {
    LTC2496_CH0,
    LTC2496_CH1,
    LTC2496_CH2,
    LTC2496_CH3,
    LTC2496_CH4,
    LTC2496_CH5,
    LTC2496_CH6,
    LTC2496_CH7,
    LTC2496_CH8,
    LTC2496_CH9,
    LTC2496_CH10,
    LTC2496_CH11,
    LTC2496_CH12,
    LTC2496_CH13,
    LTC2496_CH14,
    LTC2496_CH15};

int32_t adc_code = 0;

float readADC(int readercs, int ch)
{
  int32_t adc_code = 0;
  byte adc_command = gochans[ch]; // | LTC24XX_MULTI_CH_OSR_32768 | LTC24XX_HS_MULTI_SPEED_2X;
  LTC2496_read(readercs, LTC2496_CH11,
               &adc_code); // Obtains the current reading and stores to
                           // adc_code variable

  // Convert adc_code to voltage

  float adc_voltage = LTC2496_code_to_voltage(adc_code, READER_VREF);
  return adc_voltage;
}

void setup()
{
  int32_t adc_code = 0;
  pinMode(READER0_CS, OUTPUT);
  digitalWrite(READER0_CS, HIGH);
  pinMode(READER_CLOCK, OUTPUT);
  pinMode(READER_MOSI, OUTPUT);
  pinMode(READER_MISO, INPUT);

  quikeval_SPI_init();
  quikeval_SPI_connect();

  Serial.begin(9600);
  // put your setup code here, to run once:
  if (LTC2496_EOC_timeout(READER0_CS, MISO_TIMEOUT)) // Check for EOC
    return 1;
  LTC2496_read(READER0_CS, LTC2496_CH0, &adc_code);  // Throws out last reading
  if (LTC2496_EOC_timeout(READER0_CS, MISO_TIMEOUT)) // Check for EOC
    return 1;
}

int ticks = 0;
int lastMeasure = millis();
void loop()
{
  // put your main code here, to run repeatedly:
  ticks++;
  Serial.print("ADC values: ");

  for (int i = 0; i < 16; i++)
  {
    // delay(150);
    // LTC2496_EOC_timeout(READER0_CS, MISO_TIMEOUT);
    Serial.print(readADC(READER0_CS, i));
    Serial.print(" ");
  }
  Serial.println();

  if (millis() - lastMeasure > 1000)
  {
    //Serial.println(String(ticks) + " TPS");
    ticks = 0;
    lastMeasure = millis();
  }
  delay(500);
}
