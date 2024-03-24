#pragma once

#include <dlib/gui_widgets/widgets.h>
#include <dlib/image_processing/full_object_detection.h>
#include <dlib/image_transforms/interpolation.h>
#include <iterator>
#include <vector>

#include <dlib/image_processing/render_face_detections.h>

#include "cv_image.cpp"
#include "full_object_detection.cpp"
#include "matrix.cpp"
#include "overlay_line.cpp"

namespace wrapper {
std::vector<OverlayLine>
render_face_detections(const FullObjectDetection &full_object_detections) {
  std::vector<dlib::image_window::overlay_line> dlib_overlay_lines =
      dlib::render_face_detections(full_object_detections.inner);

  std::vector<OverlayLine> overlay_lines;

  std::transform(
      dlib_overlay_lines.begin(), dlib_overlay_lines.end(),
      std::back_inserter(overlay_lines),
      [](dlib::image_window::overlay_line x) { return OverlayLine(x); });

  return overlay_lines;
}

struct ChipDetails {
  dlib::chip_details inner;
  ChipDetails(dlib::chip_details chip_details) : inner(chip_details) {}
};

void extract_image_chip(const CvImage &cv_image,
                        const ChipDetails &chip_details, Matrix &matrix) {
  dlib::extract_image_chip(cv_image.inner, chip_details.inner, matrix.inner);
}

ChipDetails
get_face_chip_details(const FullObjectDetection &full_object_detection,
                      const unsigned long size, const double padding) {
  return ChipDetails(
      dlib::get_face_chip_details(full_object_detection.inner, size, padding));
}

} // namespace wrapper
