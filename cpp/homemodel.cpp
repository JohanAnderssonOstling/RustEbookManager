#include <qdebug.h>
#include "../headers/homemodel.h"



HomeModel::HomeModel(QObject *parent) : QAbstractListModel {parent} {
    
}
void HomeModel::addLibrary(QString name){

}
void HomeModel::deleteLibrary(int row) {

}

void HomeModel::updateLibraryList() {

}

int HomeModel::rowCount(const QModelIndex &parent) const {

}

QVariant HomeModel::data(const QModelIndex &index, int role) const {
    return "";
}

QHash<int, QByteArray> HomeModel::roleNames() const {
    static QHash<int, QByteArray> mapping {
		{UuidRole, "uuid"},
		{NameRole, "name"},
	};
	return mapping;
}