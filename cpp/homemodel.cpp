#include <qdebug.h>
#include <QQmlApplicationEngine>
#include "homemodel.h"
#include "librarymodel.h"
#include "rustutil.h"


HomeModel::HomeModel(QObject *parent, QQmlApplicationEngine *engine) : QAbstractListModel {parent} {
    HomeModel::updateLibraryList();
}
void HomeModel::addLibrary(const QString& path){
	QString name = path.split("/").last();
	beginInsertRows(QModelIndex(), libraryList.size(), libraryList.size());
	this->libraryList.push_back(add_library(name.toStdString(), path.toStdString()));
	endInsertRows();

}
void HomeModel::deleteLibrary(int row) {
	beginRemoveRows(QModelIndex(), row, row);
	delete_library(libraryList.at(row).uuid);
	updateLibraryList();
	endRemoveRows();
}

void HomeModel::updateLibraryList() {
	beginInsertRows(QModelIndex(), 0, 0);
	rust::Vec<Library> lib_vec = get_library_list();
	this->libraryList = lib_vec;
	endInsertRows();
}

int HomeModel::rowCount(const QModelIndex &parent) const {
	if (parent.isValid()) return 0;
	return this->libraryList.size();
}


QVariant HomeModel::data(const QModelIndex &index, int role) const {
	switch(role) {
		case UuidRole: return QString::fromStdString(std::string((libraryList.at(index.row()).uuid)));
		case NameRole: return QString::fromStdString(std::string((libraryList.at(index.row()).name)));
		case PathRole: return QString::fromStdString(std::string((libraryList.at(index.row()).path)));
	}
	return "";
}

QHash<int, QByteArray> HomeModel::roleNames() const {
    static QHash<int, QByteArray> mapping {
		{UuidRole, "uuid"},
		{NameRole, "name"},
		{PathRole, "path"},
	};
	return mapping;
}

void HomeModel::openLibrary(int row){
	Library library = libraryList.at(row);

	QString libraryContext = "library" + RustUtil::asQString(library.uuid);
}
