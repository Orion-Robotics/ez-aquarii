#include "nffi/include/imageprovider.h"
#include <iostream>
#include <array>
#include "raspicam/raspicam.h"
// #include <opencv2/core.hpp>
// #include <opencv2/imgcodecs.hpp>
// #include <opencv2/highgui.hpp>

using namespace std;

ImagePacket::ImagePacket(uint8_t* data, size_t len) {
	this->data = data;
	this->len = len;
}
Cam::Cam(uint32_t w, uint32_t h, uint32_t framerate) {
	this->camera = new raspicam::RaspiCam();
	camera->setWidth(w);
	camera->setHeight(h);
	camera->setFrameRate(framerate);
	this->frame_size = this->camera->getImageTypeSize(raspicam::RASPICAM_FORMAT_RGB);
	this->frame = new uint8_t[this->frame_size];
	this->camera->open();
}
Cam::~Cam() = default;
// raspicam::RaspiCam::~RaspiCam() = default;
// raspicam::RaspiCam::RaspiCam() = default;
Cam* globalCamera = NULL;


ImagePacket get_image_packet() {
	auto cam = globalCamera->camera;
	cam->grab();
    cam->retrieve(globalCamera->frame, raspicam::RASPICAM_FORMAT_RGB);
    return ImagePacket(globalCamera->frame, globalCamera->frame_size);
}
 
void initialize_camera(uint32_t w, uint32_t h, uint32_t framerate) {
	globalCamera = new Cam(w, h, framerate);
}
