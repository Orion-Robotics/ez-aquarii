#include "nffi/include/imageprovider.h"
#include <iostream>
#include "nffi/include/raspicam/src/raspicam.h"
// #include <opencv2/core.hpp>
// #include <opencv2/imgcodecs.hpp>
// #include <opencv2/highgui.hpp>

ImagePacket::ImagePacket() {}


Cam::Cam() {
	raspicam::RaspiCam cam;
	this->camera = cam;
	this->num = 498;
}
Cam::~Cam() = default;
raspicam::RaspiCam::~RaspiCam() = default;
raspicam::RaspiCam::RaspiCam() = default;

std::unique_ptr<ImagePacket> get_image_packet() {
  std::cout << "Hello from C++!" << std::endl;
  return std::unique_ptr<ImagePacket>(new ImagePacket());
}

uint32_t get_number(){
  return 727;
}

uint32_t display_image(std::string impath){
  return 0;
}

std::unique_ptr<Cam> get_camera(){
	// Cam lecamera;
	return std::unique_ptr<Cam>(new Cam());
}

uint32_t get_number_from_camera(std::unique_ptr<Cam> camera){
	return camera->num;
}

