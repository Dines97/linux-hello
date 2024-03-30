#pragma once

#include <dlib/image_processing.h>
#include <dlib/matrix/matrix_utilities.h>
#include <dlib/svm/sparse_vector.h>
#include <sstream>

namespace wrapper {

struct MatrixDescriptor {
  dlib::matrix<float, 0, 1> inner;

  MatrixDescriptor(const dlib::matrix<float, 0, 1> &matrix) : inner(matrix) {}

  const std::vector<float> to_vector() const {

    std::vector<float> vec;
    vec.reserve(inner.size());

    for (long i = 0; i < inner.size(); ++i) {
      vec.push_back(inner(i));
    }

    return vec;
  }

  const MatrixDescriptor subtract(const MatrixDescriptor &matrix) const {
    return MatrixDescriptor(this->inner - matrix.inner);
  }

  float length() const { return dlib::length(this->inner); }

  std::string debug() const {
    std::ostringstream oss;
    oss << dlib::trans(inner);
    return oss.str();
  }
};
} // namespace wrapper
