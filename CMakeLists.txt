cmake_minimum_required(VERSION 3.16)

project(EbookTest VERSION 0.1 LANGUAGES CXX)

set(CMAKE_AUTOMOC ON)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 6.2 COMPONENTS Widgets Core Quick Core5Compat REQUIRED)
find_package(Qt6 6.2 COMPONENTS WebView Sql)
find_package(Corrosion REQUIRED)
qt_add_executable(appEbookTest
    cpp/main.cpp
    cpp/homemodel.cpp

    headers/homemodel.h
)

qt_add_qml_module(appEbookTest
    URI EbookTest
    VERSION 1.0
    QML_FILES 
        qml/main.qml 
        qml/HomeView.qml 
        qml/LibraryView.qml 
        qml/EpubReader.qml
)

set_target_properties(appEbookTest PROPERTIES
    MACOSX_BUNDLE_GUI_IDENTIFIER my.example.com
    MACOSX_BUNDLE_BUNDLE_VERSION ${PROJECT_VERSION}
    MACOSX_BUNDLE_SHORT_VERSION_STRING ${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}
    MACOSX_BUNDLE TRUE
    WIN32_EXECUTABLE TRUE
)

corrosion_import_crate(MANIFEST_PATH /home/johan/Programmering/EbookTest/rust/client_lib/Cargo.toml
)

target_link_libraries(appEbookTest
    PRIVATE Qt6::Quick
    Qt6::Core
    
    PUBLIC client_lib)

install(TARGETS appEbookTest BUNDLE DESTINATION .)
