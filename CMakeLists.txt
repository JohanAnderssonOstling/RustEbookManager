cmake_minimum_required(VERSION 3.16)

project(RustEbookManger VERSION 0.1 LANGUAGES CXX)
set(HOME_DIR $ENV{HOME})
#set(CMAKE_PREFIX_PATH "${HOME_DIR}/.config/Qt-dev/6.4.2/gcc_64/lib/cmake")
#set(CMAKE_PREFIX_PATH "/home/johandost/Qt/5.15.2/gcc_64/lib/cmake")
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_INCLUDE_CURRENT_DIR ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS true)

set(CLIENT_LIB_DIR ${CMAKE_SOURCE_DIR}/rust/client_lib)
set(CLIENT_LIB_CARGO ${CLIENT_LIB_DIR}/Cargo.toml)
set(CLIENT_LIB_SRC ${CLIENT_LIB_DIR}/src)

set(CPP_DIR ${CMAKE_SOURCE_DIR}/cpp)
set(RUST_BRIDGE_CPP ${CPP_DIR}/rust_bridge.cpp)
set(H_DIR ${CMAKE_SOURCE_DIR}/headers)
set(RUST_BRIDGE_HEADER ${H_DIR}/rust_bridge.h)

set(ENV{RUST_BACKTRACE} 1)


find_package(Qt6 COMPONENTS Core Gui Quick QuickControls2 REQUIRED)
find_package(Corrosion REQUIRED)

include(corrosion_cxx.cmake)

#print CMAKE_SOURCE_DIR
message(STATUS "CMAKE_SOURCE_DIR: ${CMAKE_SOURCE_DIR}/qml/lib/QtAV")
#set(QML_IMPORT_PATH ${CMAKE_SOURCE_DIR}/qml/lib/QtAV ${CMAKE_BINARY_DIR}/imports CACHE STRING "" FORCE)

qt_add_executable(EbookTest cpp/main.cpp cpp/homemodel.cpp cpp/librarymodel.cpp cpp/homemodel.h cpp/rustutil.cpp cpp/rustutil.h
        qml/main.qml qml/HomeView.qml qml/LibraryView.qml qml/EpubReader.qml)
add_library_rust(PATH rust/client_lib NAMESPACE my)
target_link_libraries(EbookTest
        PUBLIC my::client_lib
        PRIVATE Qt6::Quick Qt6::Core ssl)
