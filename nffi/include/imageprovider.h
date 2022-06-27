#pragma once
#include <memory>
#include <iostream>
#include <string>
#include <array>
#include "raspicam/raspicam.h"
#include <opencv2/core.hpp>
#include <opencv2/imgcodecs.hpp>
#include <opencv2/highgui.hpp>

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

	ImagePacket(uint8_t* ptr, uint32_t len);
};

class Cam {
	public: 
		raspicam::RaspiCam* camera;
		size_t frame_size;
		uint8_t* frame;
		Cam(uint32_t w, uint32_t h, uint32_t framerate);
		~Cam();
};

extern Cam* globalCamera;

void initialize_camera(uint32_t w, uint32_t h, uint32_t framerate);
ImagePacket get_image_packet();
