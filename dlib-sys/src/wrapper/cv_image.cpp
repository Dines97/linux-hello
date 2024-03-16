#pragma once

#include <opencv2/core.hpp>

#include <dlib/opencv/cv_image.h>

namespace wrapper {

struct CvImage {
  dlib::cv_image<dlib::bgr_pixel> inner;

  CvImage(cv::Mat &mat) : inner(mat) {}
};
} // namespace wrapper
