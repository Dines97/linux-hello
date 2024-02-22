#pragma once

#include <opencv2/opencv.hpp>

namespace wrapper {
    void streamExtraction(cv::VideoCapture &videoCapture, cv::Mat &mat) {
        videoCapture >> mat;
    }

    void imshow(const cv::String &winname, cv::Mat &mat) {
        cv::imshow(winname, mat);
    }
}
