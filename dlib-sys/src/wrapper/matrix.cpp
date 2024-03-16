#pragma once

#include <dlib/image_processing.h>

namespace wrapper {

struct Matrix {
  dlib::matrix<dlib::rgb_pixel> inner;
};
} // namespace wrapper
