cmake_minimum_required(VERSION 3.16)

project(RustEbookManger VERSION 0.1 LANGUAGES CXX)
set(CMAKE_PREFIX_PATH "/home/johandost/Qt/6.4.2/gcc_64/lib/cmake")
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



find_package(Qt6Core REQUIRED)
find_package(Qt6Widgets REQUIRED)
find_package(WrapOpenGL REQUIRED)
find_package(Qt6 REQUIRED COMPONENTS Gui)
qt_standard_project_setup()
find_package(Qt6Quick REQUIRED)
find_package(Corrosion REQUIRED)

include(corrosion_cxx.cmake)

qt_add_executable(EbookTest
    cpp/main.cpp
    cpp/homemodel.cpp
        cpp/librarymodel.cpp
        cpp/homemodel.h
        qml/main.qml
        qml/HomeView.qml
        qml/LibraryView.qml
        qml/EpubReader.qml

        cpp/librarymodel.cpp cpp/librarymodel.h cpp/rustutil.cpp cpp/rustutil.h)



add_library_rust(PATH rust/client_lib NAMESPACE my)

target_link_libraries(EbookTest
        PUBLIC
        my::client_lib
    PRIVATE Qt6::Quick
        ssl
        crypto
    Qt6::Core
    Qt6::Widgets
    PUBLIC client_lib)

#install(TARGETS EbookTest BUNDLE DESTINATION .)
