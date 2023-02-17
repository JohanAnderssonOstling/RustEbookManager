//
// Created by johandost on 2/4/23.
//

#include "librarymodel.h"
#include "rustutil.h"

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
	return QString::fromStdString(std::string(uuid)) + "/" +
		   QString::number(getCoverWidth()) + ".jpg";
}

QVariant LibraryModel::bookData(int row, int role) const{
	qInfo() << "Getting data from" << role;
	const Book book = bookList.at(row);
	switch (role){
		case UUIDRole:
			qInfo() << "UUIDRole" << QString::fromStdString(std::string(book.uuid));
			return QString::fromStdString(std::string((book.uuid)));
		case NameRole:
			return QString::fromStdString(std::string((book.name)));
		case PathRole:
			return QString::fromStdString(std::string((book.path)));
		case AuthorRole:
			return "Placeholder";
		case HasCoverRole:
			return false;
		case CoverRole:
			return getCoverPath(row);
		case LocationRole:
			std::string location = std::string(book.read_location);
			int int_location = std::stoi(location);
			return int_location;
	}
}

QVariant LibraryModel::folderData(int row, int role) const{
	Dir dir = folderList.at(row);
	switch (role){
		case UUIDRole:
			return dir.id;
		case NameRole:
			return QString::fromStdString(std::string(dir.name));
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

void LibraryModel::openLibrary(const QString &uuid, QString url_path){
	QString path = url_path.replace("file://", "");
	rust::String rust_uuid = rust::String(uuid.toStdString());
	rust::String rust_path = rust::String(path.toStdString());
	libraryDBModel = open_library(rust_uuid, rust_path);
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
	rust::String rust_uuid = rust::String(bookUUID.toStdString());
	rust::String rust_location = rust::String(location.toStdString());
	qInfo() << "Setting location to" << location;

	libraryDBModel.at(0).set_book_location(rust_uuid, rust_location, 0);


}

