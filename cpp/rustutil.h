//
// Created by johandost on 2/4/23.
//

#ifndef RUSTEBOOKMANGER_RUSTUTIL_H
#define RUSTEBOOKMANGER_RUSTUTIL_H

#include <QString>
#include "rust/cxx.h"

namespace RustUtil{

	QString rustStringToQString(rust::Str s);
	rust::Str qStringToRustString(QString s);
}
#endif //RUSTEBOOKMANGER_RUSTUTIL_H
