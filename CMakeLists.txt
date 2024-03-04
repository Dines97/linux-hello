cmake_minimum_required(VERSION 3.28.2)
project(linux-hello)

find_package(OpenCV REQUIRED)
include_directories(${OpenCV_INCLUDE_DIRS})

find_package(Dlib REQUIRED)
include_directories(${Dlib_INCLUDE_DIRS})

# add_executable(linux-hello ${SOURCES})
# target_link_libraries(linux-hello ${OpenCV_LIBS} ${DLIB_LIBRARIES})
