#ifndef HOMEMODEL_H
#define HOMEMODEL_H

#include <QAbstractListModel>
#include <QHash>

class HomeModel : public QAbstractListModel {
	Q_OBJECT
private:
	
public:
	enum Roles {
		UuidRole = Qt::UserRole,
		NameRole,
	};
	explicit HomeModel(QObject *parent = 0);

	void updateLibraryList();
	int rowCount(const QModelIndex& parent = QModelIndex()) const override;
	QVariant data(const QModelIndex &index, int role) const override;
	QHash<int, QByteArray> roleNames() const override;

public slots:
	void addLibrary(QString name);
	void deleteLibrary(int row);

};
void test();
#endif // HOMEMODEL_H
