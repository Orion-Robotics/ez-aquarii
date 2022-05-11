#pragma once

#define TIMEOUT 500
#define LINE_SCK 13
#define LINE_MOSI 11
#define LINE_MISO 12
#define CONTROLLER_PORT Serial1
#define CONTROLLER_BAUD 500000
#define WRITE_TIMEOUT 500
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
const auto DIR_PINS = std::array<int, 4>{
    23,
    20,
    16,
    4,
};
const auto MOVE_PINS = std::array<int, 4>{
    9,
    22,
    17,
    3,
};