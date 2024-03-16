#pragma once

#include <dlib/image_processing/full_object_detection.h>

namespace wrapper {

struct FullObjectDetection {
  dlib::full_object_detection inner;

  FullObjectDetection(dlib::full_object_detection full_object_detection)
      : inner(full_object_detection) {}
};
} // namespace wrapper
