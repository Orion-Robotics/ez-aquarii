#pragma once
#include <memory>
#include <iostream>
#include <string>
#include "raspicam/raspicam.h"
// #include <opencv2/core.hpp>
// #include <opencv2/imgcodecs.hpp>
// #include <opencv2/highgui.hpp>

// class ImagePacket {
    // public:
        // ImagePacket();
        // // uint32_t get_number();
        // // uint32_t display_image();
        // // get_camera();
// };

struct ImagePacket {
	uint8_t* data;
	size_t len;

	ImagePacket(uint8_t* ptr, size_t len);
};

class Cam {
	public: 
		raspicam::RaspiCam camera;
		Cam();
		~Cam();
};

extern Cam* globalCamera;

ImagePacket get_image_packet();
