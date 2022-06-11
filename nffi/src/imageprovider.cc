#include "nffi/include/imageprovider.h"
#include <iostream>
// #include <opencv2/core.hpp>
// #include <opencv2/imgcodecs.hpp>
// #include <opencv2/highgui.hpp>

ImagePacket::ImagePacket() {}

std::unique_ptr<ImagePacket> get_image_packet() {
  std::cout << "Hello from C++!" << std::endl;
  return std::unique_ptr<ImagePacket>(new ImagePacket());
}

uint32_t get_number(){
  return 727;
}

uint32_t display_image(std::string impath){
  // Mat img = imread(impath, IMREAD_COLOR);
  // if(img.empty())
  // {
  //   std::cout << "Could not read the image: " << image_path << std::endl;
  //   return 1;
  // }
  // imshow("nya", img);
  // int k = waitKey(0);
  return 0;
}
