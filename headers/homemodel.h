#ifndef HOMEMODEL_H
#define HOMEMODEL_H

#include <qt6/QtCore/QAbstractListModel>
#include <qt6/QtCore/QHash>
#include <qt6/QtQml/QQmlApplicationEngine>
#include "rust/cxx.h"
#include "cxx_layer/src/home_model_cxx.rs.h

class HomeModel : public QAbstractListModel {
	Q_OBJECT
private:
	rust::Vec<Library> libraryList;
public:
	enum Roles {
		UuidRole = Qt::UserRole,
		NameRole,
		PathRole,
	};

	explicit HomeModel(QObject *parent = 0, QQmlApplicationEngine *engine = nullptr);
	void updateLibraryList();
	int rowCount(const QModelIndex& parent = QModelIndex()) const override;
	QVariant data(const QModelIndex &index, int role) const override;
	QHash<int, QByteArray> roleNames() const override;



public slots:
	void addLibrary(const QString& name);
	void openLibrary(int row);
	void deleteLibrary(int row);

};
void test();
#endif // HOMEMODEL_H
