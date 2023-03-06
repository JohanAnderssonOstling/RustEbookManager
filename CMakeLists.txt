cmake_minimum_required(VERSION 3.16)

project(RustEbookManger VERSION 0.1 LANGUAGES CXX)
set(HOME_DIR $ENV{HOME})
set(CMAKE_PREFIX_PATH "${HOME_DIR}/.config/Qt-Dev/6.4.2/wasm32/lib/cmake")

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

set(ENV{RUST_BACKTRACE} 0)


find_package(Qt6 COMPONENTS Core Gui Quick QuickControls2 REQUIRED)
find_package(Corrosion REQUIRED)

include(corrosion_cxx.cmake)

file(GLOB_RECURSE cpp ${CMAKE_SOURCE_DIR}/cpp*.cpp)
file(GLOB_RECURSE headers ${CMAKE_SOURCE_DIR}/headers/*.h)
file(GLOB_RECURSE qml ${CMAKE_SOURCE_DIR}/qml/*.qml)

qt_add_executable(EbookTest ${cpp} ${headers} ${qml} cpp/librarymodel.cpp cpp/rustutil.cpp cpp/homemodel.cpp cpp/main.cpp)

option(WASM "Build for WASM" OFF)
if (WASM)
    set(CMAKE_TOOLCHAIN_FILE ${CMAKE_SOURCE_DIR}/emsdk/upstream/emscripten/cmake/Modules/Platform/Emscripten.cmake)
    set(CMAKE_EXECUTABLE_SUFFIX ".wasm.js")
    set_target_properties(EbookTest PROPERTIES LINK_FLAGS "-s WASM=1 -s EXPORTED_FUNCTIONS='[_main]'")
    add_library_rust(PATH rust/wasm_layer NAMESPACE my)
    target_link_libraries(EbookTest
            PUBLIC my::wasm_layer
            PRIVATE Qt6::Quick Qt6::Core ssl)
else()
    add_library_rust(PATH rust/cxx_layer NAMESPACE my)
    target_link_libraries(EbookTest
            PUBLIC my::cxx_layer
            PRIVATE Qt6::Quick Qt6::Core ssl)
endif()
