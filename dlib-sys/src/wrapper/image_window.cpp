#pragma once

#include <dlib/gui_widgets.h>

#include "cv_image.cpp"
#include "rectangle.cpp"

namespace wrapper {

struct ImageWindow {
  dlib::image_window inner;

  void setImage(CvImage &cv_image) { inner.set_image(cv_image.inner); }

  void addOverlay(Rectangle &rectangle) {
    this->inner.add_overlay(rectangle.inner);
  }

  void addOverlay(std::vector<Rectangle> rectangles) {
    std::vector<dlib::rectangle> dlib_rectangles;

    std::transform(rectangles.begin(), rectangles.end(),
                   std::back_inserter(dlib_rectangles),
                   [](Rectangle x) { return x.inner; });

    this->inner.add_overlay(dlib_rectangles);
  }

  void clearOverlay() { this->inner.clear_overlay(); }
};
} // namespace wrapper
