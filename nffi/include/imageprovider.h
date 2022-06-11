#pragma once
#include <memory>
#include <iostream>
#include <string>
// #include <opencv2/core.hpp>
// #include <opencv2/imgcodecs.hpp>
// #include <opencv2/highgui.hpp>

class ImagePacket {
    public: 
        ImagePacket();
        uint32_t get_number();
        uint32_t display_image();
};
std::unique_ptr<ImagePacket> get_image_packet();
uint32_t get_number();
uint32_t display_image(std::string impath);