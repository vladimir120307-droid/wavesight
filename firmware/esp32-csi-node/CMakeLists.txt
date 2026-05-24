# Top-level CMakeLists for the WaveSight ESP32-S3 CSI node firmware.
# Builds via:
#   idf.py set-target esp32s3
#   idf.py build flash monitor

cmake_minimum_required(VERSION 3.16)

include($ENV{IDF_PATH}/tools/cmake/project.cmake)

project(wavesight_csi_node)
