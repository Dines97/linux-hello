#pragma once

#include <memory>
#include <vector>

#include <dlib/image_processing/shape_predictor.h>

#include "cv_image.cpp"
#include "full_object_detection.cpp"
#include "rectangle.cpp"

namespace wrapper {

struct ShapePredictor {
  dlib::shape_predictor inner;

  ShapePredictor() {
    dlib::deserialize("shape_predictor_68_face_landmarks.dat") >> inner;
  }

  FullObjectDetection
  function_call(const CvImage &cv_image, Rectangle rectangle) const {
    return FullObjectDetection(const_cast<dlib::shape_predictor &>(inner)(cv_image.inner, rectangle.inner));
  }

  // std::vector<Rectangle> functionCall(CvImage &cv_image) {
  //   std::vector<dlib::rectangle> dlib_rectangles = inner(cv_image.inner);
  //   std::vector<Rectangle> rectangles;
  //
  //   std::transform(dlib_rectangles.begin(), dlib_rectangles.end(),
  //                  std::back_inserter(rectangles),
  //                  [](dlib::rectangle x) { return Rectangle(x); });
  //
  //   return rectangles;
  // }
};
} // namespace wrapper
