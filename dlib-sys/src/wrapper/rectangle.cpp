#pragma once

#include <dlib/geometry/rectangle.h>

namespace wrapper {

struct Rectangle {
  const dlib::rectangle inner;

  Rectangle(dlib::rectangle rectangle) : inner(rectangle) {}

  // Not sure how to implement copy or clone train on rust side
  // TODO: use actual copy constructor
  Rectangle clone() const { return *this; }
};
} // namespace wrapper
