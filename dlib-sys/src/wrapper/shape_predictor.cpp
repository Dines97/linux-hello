#pragma once

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

  std::vector<FullObjectDetection>
  functionCall(CvImage &cv_image, std::vector<Rectangle> rectangles) {
    std::vector<FullObjectDetection> full_object_detections;

    std::transform(rectangles.begin(), rectangles.end(),
                   std::back_inserter(full_object_detections),
                   [this, &cv_image](Rectangle x) {
                     return inner(cv_image.inner, x.inner);
                   });

    return full_object_detections;
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
