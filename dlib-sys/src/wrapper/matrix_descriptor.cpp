#pragma once

#include <dlib/image_processing.h>

namespace wrapper {

struct MatrixDescriptor {
  dlib::matrix<float, 0, 1> inner;

  MatrixDescriptor(const dlib::matrix<float, 0, 1> &matrix) : inner(matrix) {}
};
} // namespace wrapper
