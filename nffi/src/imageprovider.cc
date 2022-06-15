#include "nffi/include/imageprovider.h"
#include <iostream>
#include "raspicam/raspicam.h"
// #include <opencv2/core.hpp>
// #include <opencv2/imgcodecs.hpp>
// #include <opencv2/highgui.hpp>


ImagePacket::ImagePacket(uint8_t* data, size_t len) {
	this->data = data;
	this->len = len;
}
Cam::Cam() {
	raspicam::RaspiCam cam;
	this->camera = cam;
}
Cam::~Cam() = default;
// raspicam::RaspiCam::~RaspiCam() = default;
// raspicam::RaspiCam::RaspiCam() = default;
Cam* globalCamera = NULL;

ImagePacket get_image_packet() {
	raspicam::RaspiCam cam = globalCamera->camera;
	cam.grab();
	auto len = cam.getImageTypeSize(raspicam::RASPICAM_FORMAT_RGB);
    unsigned char *data = new unsigned char[len];
    cam.retrieve (data, raspicam::RASPICAM_FORMAT_RGB);

    auto img = ImagePacket(data, len);

    return img;
}
 
