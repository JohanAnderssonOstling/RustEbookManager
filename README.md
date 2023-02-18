# Rust Ebook Manager

This is a simple ebook and pdf manager written in Rust and C++.

## Dependencies

### Qt 6.4.2
- Core
- Gui
- Quick
- WebView
- QuickControls2
- Pdf

### Corrosion

### Rust

### CMake 3.16

## Building

To build the project, run the following commands:

```bash
mkdir build
cd build
cmake -G "Unix Makefiles" ../ 
make
```

## Running

To run the project, run: 

```bash
./EbookTest
```

## Features
- Create a library by importing a directory of ebooks and pdfs.
- Displays the cover of the ebook or pdf when browsing a library. 
- Remembers the active page of a document when closing it, and opens it to that page when opened again.

## Design
