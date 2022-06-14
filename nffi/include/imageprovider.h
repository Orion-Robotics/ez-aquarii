#pragma once
#include <memory>
#include <iostream>
#include <string>
#include "nffi/include/raspicam/src/raspicam.h"
// #include <opencv2/core.hpp>
// #include <opencv2/imgcodecs.hpp>
// #include <opencv2/highgui.hpp>

class ImagePacket {
    public: 
        ImagePacket();
        // uint32_t get_number();
        // uint32_t display_image();
        // get_camera();
};

class Cam {
	public: 
		raspicam::RaspiCam camera;
		Cam();
		~Cam();
		uint32_t num;
};
std::unique_ptr<ImagePacket> get_image_packet();
uint32_t get_number();
uint32_t display_image(std::string impath);
std::unique_ptr<Cam> get_camera();
uint32_t get_number_from_camera(std::unique_ptr<Cam> camera);
