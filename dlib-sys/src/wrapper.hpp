#pragma once

#include <dlib/conditioning_class.h>
#include <dlib/geometry/rectangle.h>
#include <dlib/image_processing/frontal_face_detector.h>
#include <dlib/image_processing/full_object_detection.h>
#include <dlib/image_transforms/assign_image.h>
#include <dlib/opencv/cv_image.h>

#include <opencv2/core/mat.hpp>
#include <opencv2/videoio.hpp>

#include <dlib/gui_widgets.h>
#include <dlib/image_processing.h>
#include <dlib/image_processing/frontal_face_detector.h>
#include <dlib/image_processing/render_face_detections.h>
#include <dlib/opencv.h>
#include <opencv2/highgui/highgui.hpp>
#include <vector>

#include <dlib/dnn.h>
#include <opencv2/core.hpp>

namespace wrapper {

struct FullObjectDetection {
  dlib::full_object_detection inner;

  FullObjectDetection(dlib::full_object_detection full_object_detection)
      : inner(full_object_detection) {}
};

struct Rectangle {
  dlib::rectangle inner;

  Rectangle(dlib::rectangle rectangle) : inner(rectangle) {}
};

struct Matrix {
  dlib::matrix<dlib::rgb_pixel> inner;
};

struct CvImage {
  dlib::cv_image<dlib::bgr_pixel> inner;

  CvImage(cv::Mat &mat) : inner(mat) {}
};

struct FrontalFaceDetector {
  dlib::frontal_face_detector inner;

  FrontalFaceDetector() : inner(dlib::get_frontal_face_detector()) {}

  std::vector<Rectangle> functionCall(CvImage &cv_image) {
    std::vector<dlib::rectangle> dlib_rectangles = inner(cv_image.inner);

    std::vector<Rectangle> rectangles;
    for (const auto &dlib_rectangle : dlib_rectangles) {
      rectangles.push_back(Rectangle(dlib_rectangle));
    }

    return rectangles;
  }
};

struct ImageWindow {
  dlib::image_window inner;

  void setImage(CvImage &cv_image) { inner.set_image(cv_image.inner); }

  void addOverlay(Rectangle &rectangle) {
    this->inner.add_overlay(rectangle.inner);
  }

  void addOverlay(std::vector<Rectangle> &rectangles) {

    std::vector<dlib::rectangle> dlib_rectangles;
    for (const auto &rectangle : rectangles) {
      dlib_rectangles.push_back(rectangle.inner);
    }

    this->inner.add_overlay(dlib_rectangles);
  }

  void clearOverlay() { this->inner.clear_overlay(); }
};

void capabilities() {

#ifdef DLIB_USE_CUDA
  std::cout << "Dlib is compiled with CUDA support." << std::endl;
#else
  std::cout << "Dlib is not compiled with CUDA support." << std::endl;
#endif

#ifdef DLIB_USE_BLAS
  std::cout << "Dlib is compiled with BLAS support." << std::endl;
#else
  std::cout << "Dlib is not compiled with BLAS support." << std::endl;
#endif

#ifdef DLIB_USE_LAPACK
  std::cout << "Dlib is compiled with LAPACK support." << std::endl;
#else
  std::cout << "Dlib is not compiled with LAPACK support." << std::endl;
#endif
}

} // namespace wrapper
