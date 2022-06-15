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
	this->frame_size = this->camera->getImageTypeSize(raspicam::RASPICAM_FORMAT_RGB);
	this->frame = new uint8_t[this->frame_size];
}
Cam::~Cam() = default;
// raspicam::RaspiCam::~RaspiCam() = default;
// raspicam::RaspiCam::RaspiCam() = default;
Cam* globalCamera = NULL;


ImagePacket get_image_packet() {
	auto cam = globalCamera->camera;
	cout << "got cam" << endl;
	cam->grab();
	cout << "grabbed frame" << endl;
    cam->retrieve (globalCamera->frame, raspicam::RASPICAM_FORMAT_RGB);
	cout << "received frame" << endl;
	cout << "frame size" << globalCamera->frame_size << endl;
    return ImagePacket(globalCamera->frame, globalCamera->frame_size);
}
 
void initialize_camera() {
	cout << "new cam" << endl;
	globalCamera = new Cam();
	cout << "exit initialize" << endl;
}
