#pragma once

#include <dlib/gui_widgets/widgets.h>

namespace wrapper {

struct OverlayLine {
  dlib::image_window::overlay_line inner;

  OverlayLine(dlib::image_window::overlay_line &overlay_line)
      : inner(overlay_line) {}
};
} // namespace wrapper
