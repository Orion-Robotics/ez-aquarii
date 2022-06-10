#pragma once
#include <memory>

class ImagePacket {
    public: ImagePacket();
    public: int32_t number;
};

std::unique_ptr<ImagePacket> get_image_packet();