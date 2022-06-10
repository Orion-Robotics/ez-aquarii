#include "nffi/include/imageprovider.h"

ImagePacket::ImagePacket() {}

std::unique_ptr<ImagePacket> get_image_packet() {
  return std::unique_ptr<ImagePacket>(new ImagePacket());
}

