#include "headers/librarymodel.h"
#include "headers/rustutil.h"

using namespace RustUtil;
using namespace std;

LibraryModel::LibraryModel(QObject* parent):QAbstractListModel(parent) {
	coverWidths = get_cover_widths();
}

int LibraryModel::rowCount(const QModelIndex& parent) const {
	if (parent.isValid()) return 0;
	return folderList.size() + bookList.size();
}

QVariant LibraryModel::data(const QModelIndex& index, int role) const {
	int row = index.row();
	if (row>=this->folderList.size()) return bookData(row-this->folderList.size(), role);
	else return folderData(row, role);
}

QString LibraryModel::getCoverPath(int row) const {
	rust::String uuid = get_cover_path(model_uuid, bookList.at(row).uuid);
	return asQString(uuid) + "/" + QString::number(getCoverWidth()) + ".jpg";
}

QVariant LibraryModel::bookData(int row, int role) const {
	const Book book = bookList.at(row);
	switch (role) {
	case UUIDRole: return asQString(book.uuid);
	case NameRole: return asQString(book.name);
	case PathRole: return asQString(book.path);
	case AuthRole: return "Placeholder";
	case CoverRole: return getCoverPath(row);
	case HasCoverRole: return true;
	case LocationRole: return asInt(get_book_location(model_uuid, book.uuid).read_location);
	default: throw runtime_error("Undefined bookrole");
	}
}

QVariant LibraryModel::folderData(int row, int role) const {
	const Dir dir = folderList.at(row);
	switch (role) {
	case UUIDRole: return dir.id;
	case NameRole: return asQString(dir.name);
	case HasCoverRole: return false;
	default: throw runtime_error("Undefined dirrole");
	}
}

QHash<int, QByteArray> LibraryModel::roleNames() const {
	return {
		{ UUIDRole, "uuid" },
		{ NameRole, "name" },
		{ PathRole, "path" },
		{ LocationRole, "location" },
		{ HasCoverRole, "hasCover" },
		{ CoverRole, "cover" },
		{ AuthRole, "author" }
	};
}

void LibraryModel::openLibrary(QString library_uuid, QString library_path) {
	this->model_uuid = asRustString(library_uuid);
	open_library(model_uuid, asRustString(library_path));
	this->changeFolder(0);
}

void LibraryModel::changeFolder(int folderID) {
	beginResetModel();
	this->folderList = get_folders(model_uuid, folderID);
	this->bookList = get_books(model_uuid, folderID);
	endResetModel();
	navStack.push(folderID);
}

bool LibraryModel::prevFolder() {
	if (navStack.size()==1) return false;
	navStack.pop();
	changeFolder(navStack.pop());
	return true;
}

bool LibraryModel::isFolder(int index) {
	return index<folderList.size();
}

void LibraryModel::setCoverWidthIndex(int coverWidthIndex) {
	if (coverWidthIndex<0 || coverWidthIndex>=coverWidths.size()) return;
	emit coverWidthChanged();
}

int LibraryModel::getCoverWidth() const {
	return coverWidths.at(coverWidthIndex);
}

void LibraryModel::setBookLocation(const QString& bookUUID, const QString& location, int percentage) {
	set_book_location(model_uuid, asRustString(bookUUID), asRustString(location), 0);
}
