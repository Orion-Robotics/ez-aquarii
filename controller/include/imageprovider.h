#include <array>
#include <iostream>

#include "raspicam/raspicam.h"
// #include <opencv2/core.hpp>
// #include <opencv2/imgcodecs.hpp>
// #include <opencv2/highgui.hpp>

using namespace std;

struct ImagePacket {
  uint8_t* data;
  size_t len;

  ImagePacket(uint8_t* ptr, uint32_t len) : data(ptr), len(len) {}
};

class Cam {
 public:
  raspicam::RaspiCam* camera;
  size_t frame_size;
  uint8_t* frame;
  Cam() {
    this->camera = new raspicam::RaspiCam();
  }
  ~Cam() = default;
};

Cam* globalCamera = NULL;

ImagePacket get_image_packet() {
  auto cam = globalCamera->camera;
  cam->grab();
  return ImagePacket(cam->getImageBufferData(), cam->getImageBufferSize());
}

void initialize_camera(
    uint32_t w,
    uint32_t h,
    uint32_t framerate,
    uint8_t sensor_mode,
    uint32_t shutter_speed,
    float awb_r,
    float awb_b,
    int32_t saturation,
    uint32_t brightness,
    int32_t exposure,
    int32_t iso) {
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
  camera->setISO(iso);
  camera->setSaturation(saturation);
  camera->setExposureCompensation(exposure);
  camera->setAWB_RB(awb_r, awb_b);
  camera->setBrightness(brightness);
  camera->open();
}
