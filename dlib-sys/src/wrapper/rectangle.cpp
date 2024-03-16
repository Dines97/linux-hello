#pragma once

#include <dlib/geometry/rectangle.h>

namespace wrapper {

struct Rectangle {
  dlib::rectangle inner;

  Rectangle(dlib::rectangle rectangle) : inner(rectangle) {}
};
} // namespace wrapper
