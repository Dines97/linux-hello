#pragma once

#include <dlib/clustering.h>
#include <dlib/dnn.h>
#include <dlib/gui_widgets.h>
#include <dlib/image_io.h>
#include <dlib/image_processing.h>
#include <dlib/image_processing/frontal_face_detector.h>
#include <dlib/string.h>

#include "matrix.cpp"
#include "matrix_descriptor.cpp"
#include "rectangle.cpp"
#include "rust/cxx.h"

namespace dlib_ext {
using namespace dlib;
template <template <int, template <typename> class, int, typename> class block,
          int N, template <typename> class BN, typename SUBNET>
using residual = add_prev1<block<N, BN, 1, tag1<SUBNET>>>;

template <template <int, template <typename> class, int, typename> class block,
          int N, template <typename> class BN, typename SUBNET>
using residual_down =
    add_prev2<avg_pool<2, 2, 2, 2, skip1<tag2<block<N, BN, 2, tag1<SUBNET>>>>>>;

template <int N, template <typename> class BN, int stride, typename SUBNET>
using block =
    BN<con<N, 3, 3, 1, 1, relu<BN<con<N, 3, 3, stride, stride, SUBNET>>>>>;

template <int N, typename SUBNET>
using ares = relu<residual<block, N, affine, SUBNET>>;
template <int N, typename SUBNET>
using ares_down = relu<residual_down<block, N, affine, SUBNET>>;

template <typename SUBNET> using alevel0 = ares_down<256, SUBNET>;
template <typename SUBNET>
using alevel1 = ares<256, ares<256, ares_down<256, SUBNET>>>;
template <typename SUBNET>
using alevel2 = ares<128, ares<128, ares_down<128, SUBNET>>>;
template <typename SUBNET>
using alevel3 = ares<64, ares<64, ares<64, ares_down<64, SUBNET>>>>;
template <typename SUBNET> using alevel4 = ares<32, ares<32, ares<32, SUBNET>>>;

using anet_type = loss_metric<fc_no_bias<
    128,
    avg_pool_everything<alevel0<alevel1<alevel2<alevel3<alevel4<max_pool<
        3, 3, 2, 2,
        relu<affine<con<32, 7, 7, 2, 2, input_rgb_image_sized<150>>>>>>>>>>>>>;

} // namespace dlib_ext

namespace wrapper {

struct FaceRecognitionResnetModelV1 {
  dlib_ext::anet_type inner;

  FaceRecognitionResnetModelV1(const std::string file_path) {
    dlib::deserialize(file_path) >> inner;
  }

  MatrixDescriptor function_call(const Matrix &matrix) {

    std::vector<dlib::matrix<dlib::rgb_pixel>> dlib_matricies = {matrix.inner};

    // std::cout << "resnet working?" << std::endl;

    std::vector<dlib::matrix<float, 0, 1>> dlib_face_descriptors =
        inner(dlib_matricies);

    return MatrixDescriptor(dlib_face_descriptors.front());
  }

  // std::vector<MatrixDescriptor>
  // function_call(rust::Vec<Matrix> matricies) const {
  //   std::vector<dlib::matrix<dlib::rgb_pixel>> dlib_matricies;
  //   std::transform(matricies.begin(), matricies.end(),
  //                  std::back_inserter(dlib_matricies),
  //                  [](Matrix x) { return x.inner; });
  //
  //   std::vector<dlib::matrix<float, 0, 1>> dlib_face_descriptors =
  //       const_cast<dlib_ext::anet_type &>(inner)(dlib_matricies);
  //
  //   std::vector<MatrixDescriptor> face_descriptors;
  //   std::transform(
  //       dlib_face_descriptors.begin(), dlib_face_descriptors.end(),
  //       std::back_inserter(face_descriptors),
  //       [](dlib::matrix<float, 0, 1> x) { return MatrixDescriptor(x); });
  //
  //   return face_descriptors;
  // }
};
} // namespace wrapper
