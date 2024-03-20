#pragma once

#include <cstddef>
#include <cstdint>
#include <dlib/image_processing/frontal_face_detector.h>
#include <dlib/image_processing/full_object_detection.h>
#include <dlib/image_processing/shape_predictor.h>
#include <memory>

#include "cv_image.cpp"
#include "rectangle.cpp"

namespace wrapper {

// I don't know how to get UniquePtr<Rectangle> on rust side from
// CxxVector<Rectangle> It's a reason for this terrible code
struct RectanglesWrapper {
  std::vector<std::shared_ptr<Rectangle>> inner;

  RectanglesWrapper(std::vector<std::shared_ptr<Rectangle>> inner)
      : inner(inner) {}

  size_t size() const { return inner.size(); }

  std::shared_ptr<Rectangle> pop() {
    auto x = inner.back();
    inner.pop_back();
    return x;
  }
};

struct FrontalFaceDetector {
  const dlib::frontal_face_detector inner;

  FrontalFaceDetector() : inner(dlib::get_frontal_face_detector()) {}

  RectanglesWrapper function_call(CvImage &cv_image) const {
    std::vector<dlib::rectangle> dlib_rectangles =
        const_cast<dlib::frontal_face_detector &>(inner)(cv_image.inner);
    std::vector<std::shared_ptr<Rectangle>> rectangles;

    std::transform(dlib_rectangles.begin(), dlib_rectangles.end(),
                   std::back_inserter(rectangles), [](dlib::rectangle x) {
                     return std::shared_ptr<Rectangle>(new Rectangle(x));
                   });

    return RectanglesWrapper(rectangles);
  }
};

} // namespace wrapper
