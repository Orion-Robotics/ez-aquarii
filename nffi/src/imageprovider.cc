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
Cam::Cam() {
	this->camera = new raspicam::RaspiCam();
}
Cam::~Cam() = default;
// raspicam::RaspiCam::~RaspiCam() = default;
// raspicam::RaspiCam::RaspiCam() = default;
Cam* globalCamera = NULL;


ImagePacket get_image_packet() {
	auto cam = globalCamera->camera;
	cam->grab();
    return ImagePacket(cam->getImageBufferData(), cam->getImageBufferSize());
}
 
void initialize_camera(uint32_t w, uint32_t h, uint32_t framerate, uint8_t sensor_mode, uint32_t shutter_speed) {
	globalCamera = new Cam();
	auto camera = globalCamera->camera;
	camera->setWidth(w);
	camera->setHeight(h);
	camera->setFrameRate(framerate);
	camera->setFormat(raspicam::RASPICAM_FORMAT_RGB);
	camera->setSensorMode(sensor_mode);
	camera->setShutterSpeed(shutter_speed);
	camera->setAWB(raspicam::RASPICAM_AWB_OFF);
	camera->setExposure(raspicam::RASPICAM_EXPOSURE_OFF);
	camera->setISO(500);
	camera->open();
}


