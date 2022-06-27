#include <Arduino.h>

void setup() {
  // put your setup code here, to run once:
  pinMode(9, OUTPUT);
  pinMode(22, OUTPUT);
  pinMode(17, OUTPUT);
  pinMode(3, OUTPUT);
  pinMode(9, OUTPUT);
  pinMode(22, OUTPUT);
  pinMode(17, OUTPUT);
  pinMode(3, OUTPUT);
  
}

void loop() {
  // Serial.println("sus");
  // for (int i = 0; i < 255; i++){
    analogWrite(9, 127); //FR
    analogWrite(23, 255);
    analogWrite(22, 127); //FL
    analogWrite(20, 0);
    analogWrite(17, 127); //BL
    analogWrite(16, 255);
    analogWrite(3, 127); //BR
    analogWrite(4, 0);
  //   delay(10);
  //   Serial.println(i);
  // }
  // analogWrite(22, 255);
  // analogWrite(17, 255);
  // analogWrite(3, 255);
  // analogWrite(9, 127); //FR
  // analogWrite(23, 255);
  // delay(400);
  // // analogWrite(9, 255);
  // // delay(10);
  // analogWrite(23, 0);
  // analogWrite(9, 127);
  // delay(400);f
}