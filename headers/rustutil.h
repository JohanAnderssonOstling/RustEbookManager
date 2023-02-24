//
// Created by johandost on 2/4/23.
//

#ifndef RUSTEBOOKMANGER_RUSTUTIL_H
#define RUSTEBOOKMANGER_RUSTUTIL_H

#include <qt6/QtCore/QString>
#include "rust/cxx.h"

namespace RustUtil{

	QString asQString(rust::Str s);
	rust::String asRustString(QString s);
	int asInt(rust::Str s);
}
#endif //RUSTEBOOKMANGER_RUSTUTIL_H
