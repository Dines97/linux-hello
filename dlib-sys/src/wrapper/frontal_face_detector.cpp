#pragma once

#include <cstddef>
#include <cstdint>
#include <dlib/image_processing/frontal_face_detector.h>
#include <dlib/image_processing/full_object_detection.h>
#include <dlib/image_processing/shape_predictor.h>
#include <memory>
#include <vector>

#include "cv_image.cpp"
#include "rectangle.cpp"

namespace wrapper {

struct FrontalFaceDetector {
  dlib::frontal_face_detector inner;

  FrontalFaceDetector() : inner(dlib::get_frontal_face_detector()) {}

  std::vector<Rectangle> function_call(const CvImage &cv_image) {
    std::vector<dlib::rectangle> dlib_rectangles = inner(cv_image.inner);
    std::vector<Rectangle> rectangles;

    std::transform(dlib_rectangles.begin(), dlib_rectangles.end(),
                   std::back_inserter(rectangles),
                   [](dlib::rectangle x) { return Rectangle(x); });

    return rectangles;
  }
};

} // namespace wrapper
