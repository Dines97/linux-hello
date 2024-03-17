#pragma once

#include <dlib/gui_widgets/widgets.h>
#include <dlib/image_processing/full_object_detection.h>
#include <iterator>
#include <vector>

#include <dlib/image_processing/render_face_detections.h>

#include "full_object_detection.cpp"
#include "overlay_line.cpp"

namespace wrapper {
std::vector<OverlayLine>
renderFaceDetections(std::vector<FullObjectDetection> full_object_detections) {

  std::vector<dlib::full_object_detection> dlib_full_object_detections;

  std::transform(full_object_detections.begin(), full_object_detections.end(),
                 std::back_inserter(dlib_full_object_detections),
                 [](FullObjectDetection x) { return x.inner; });

  std::vector<dlib::image_window::overlay_line> dlib_overlay_lines =
      dlib::render_face_detections(dlib_full_object_detections);

  std::vector<OverlayLine> overlay_lines;

  std::transform(
      dlib_overlay_lines.begin(), dlib_overlay_lines.end(),
      std::back_inserter(overlay_lines),
      [](dlib::image_window::overlay_line x) { return OverlayLine(x); });

  return overlay_lines;
}

} // namespace wrapper
