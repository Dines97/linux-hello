#pragma once

#include <dlib/image_processing.h>

namespace wrapper {

struct Matrix {
  dlib::matrix<dlib::rgb_pixel> inner;

  Matrix() {}
  Matrix(const dlib::matrix<dlib::rgb_pixel> matrix) : inner(matrix) {}
};
} // namespace wrapper
