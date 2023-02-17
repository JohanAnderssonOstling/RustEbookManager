//
// Created by johandost on 2/4/23.
//

#include "rustutil.h"



QString RustUtil::asQString(rust::Str s){
	std::string std_str = std::string(s);
	return QString::fromStdString(std_str);
}

rust::Str RustUtil::asRustStr(QString s){
	return rust::Str(s.toStdString());
}

int RustUtil::asInt(rust::Str s){
	std::string std_str = std::string(s);
	return std::stoi(std_str);
}