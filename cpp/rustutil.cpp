//
// Created by johandost on 2/4/23.
//

#include "rustutil.h"



QString RustUtil::rustStringToQString(rust::Str str){
	std::string std_str = std::string(str);
	return QString::fromStdString(std_str);
}

rust::Str RustUtil::qStringToRustString(QString str){
	return rust::Str(str.toStdString());
}

