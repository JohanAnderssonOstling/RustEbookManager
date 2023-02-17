//
// Created by johandost on 2/4/23.
//

#include "librarymodel.h"
#include "rustutil.h"
using namespace RustUtil;
LibraryModel::LibraryModel(QObject *parent) : QAbstractListModel(parent){
	coverWidths = get_cover_widths();
}

int LibraryModel::rowCount(const QModelIndex &parent) const{
	if (parent.isValid()) return 0;
	return this->folderList.size() + this->bookList.size();
}

QVariant LibraryModel::data(const QModelIndex &index, int role) const{
	int row = index.row();
	try{
		if (row >= this->folderList.size()){
			return bookData(row - this->folderList.size(), role);
		} else{
			return folderData(row, role);
		}
	} catch (std::out_of_range &e){
		qInfo() << "Out of range, index is" << row << " and folder size is "
				<< this->folderList.size() << " and book size is "
				<< this->bookList.size();
	}
	return QVariant();
}

QString LibraryModel::getCoverPath(int row) const{
	rust::String uuid = libraryDBModel.at(0).get_cover_path(
			bookList.at(row).uuid);
	return asQString(uuid) + "/" +
		   QString::number(getCoverWidth()) + ".jpg";
}

QVariant LibraryModel::bookData(int row, int role) const{
	qInfo() << "Getting data from" << role;
	const Book book = bookList.at(row);
	switch (role){
		case UUIDRole:
			qInfo() << "UUIDRole" << asQString(book.uuid);
			return asQString(book.uuid);
		case NameRole:
			return asQString(book.name);
		case PathRole:
			return asQString(book.path);
		case AuthorRole:
			return "Placeholder";
		case HasCoverRole:
			return false;
		case CoverRole:
			return getCoverPath(row);
		case LocationRole:
			return asInt(book.read_location);

	}
}

QVariant LibraryModel::folderData(int row, int role) const{
	Dir dir = folderList.at(row);
	switch (role){
		case UUIDRole:
			return dir.id;
		case NameRole:
			return asQString(dir.name);
		case HasCoverRole:
			return false;
		default:
			return {};
	}
}

QHash<int, QByteArray> LibraryModel::roleNames() const{
	static QHash<int, QByteArray> mapping{
			{UUIDRole,     "uuid"},
			{NameRole,     "name"},
			{PathRole,     "path"},
			{LocationRole, "location"},
			{HasCoverRole, "hasCover"},
			{CoverRole,    "cover"},
			{AuthorRole,   "author"}
	};
	return mapping;
}

void LibraryModel::openLibrary(const QString &uuid, const QString& path){
	libraryDBModel = open_library(asRustStr(uuid), asRustStr(path));
	this->changeFolder(0);
}

void LibraryModel::changeFolder(int folderID){
	beginResetModel();
	this->folderList = libraryDBModel.at(0).get_folders(folderID);
	this->bookList = libraryDBModel.at(0).get_books(folderID);
	endResetModel();
	navStack.push(folderID);
}

bool LibraryModel::prevFolder(){
	if (navStack.size() == 1) return false;
	navStack.pop();
	changeFolder(navStack.pop());
	return true;
}

bool LibraryModel::isFolder(int index){
	return index < folderList.size();
}

void LibraryModel::setCoverWidthIndex(int coverWidthIndex){
	if (coverWidthIndex < 0 || coverWidthIndex >= coverWidths.size()) return;
	emit coverWidthChanged();
}

int LibraryModel::getCoverWidth() const{
	return coverWidths.at(coverWidthIndex);
}

void LibraryModel::	setBookLocation(const QString& bookUUID, const QString& location, int percentage){
	libraryDBModel.at(0).set_book_location(asRustStr(bookUUID), asRustStr(location), 0);
}

