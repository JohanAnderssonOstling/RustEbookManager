//
// Created by johandost on 2/4/23.
//

#ifndef RUSTEBOOKMANGER_LIBRARYMODEL_H
#define RUSTEBOOKMANGER_LIBRARYMODEL_H
#include <qt6/QtCore/QAbstractListModel>
#include <qt6/QtCore/QStack>
#include "rust/cxx.h"
#include "cxx_layer/src/library_model_cxx.rs.h"


class LibraryModel : public QAbstractListModel {
	Q_OBJECT
	Q_PROPERTY(int coverWidthProp READ getCoverWidth NOTIFY coverWidthChanged)
private:
	rust::vec<Book> bookList;
	rust::vec<Dir> folderList;
	rust::vec<uint32_t> coverWidths;
	int coverWidthIndex = 2;
	rust::String model_uuid;
	QStack<int> navStack;

public:
	enum Roles {
		UUIDRole = Qt::UserRole,
		NameRole,
		PathRole,
		AuthRole,
		LocationRole,
		HasCoverRole,
		CoverRole
	};
	enum ContainerTypes {
		FolderType,
		AuthorType,
		SubjectType,
		CollectionType
	};
	explicit LibraryModel(QObject *parent = 0);

	int rowCount(const QModelIndex& parent = QModelIndex()) const override;
	QVariant data(const QModelIndex &index, int role) const override;
	QHash<int, QByteArray> roleNames() const override;
	Q_SIGNAL void coverWidthChanged();
	int getCoverWidth() const;

public slots:
	void openLibrary(QString library_uuid, QString library_path);
	[[nodiscard]] QVariant bookData(int row, int role) const;
	QVariant folderData(int row, int role) const;
	void changeFolder(int folderID);
	bool prevFolder();
	bool isFolder(int index);
	QString getCoverPath(int row) const;
	void setCoverWidthIndex(int coverWidthIndex);
	void setBookLocation(const QString& bookUUID, const QString& location, int percentage);
};
#endif //RUSTEBOOKMANGER_LIBRARYMODEL_H
