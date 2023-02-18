import QtQuick 2.0
import QtQuick.Controls 2.5
import QtQuick.Layouts 1.3
import Qt.labs.platform 1.1
import johandost.LibraryModel 1.0

ColumnLayout{
	property bool showAddButton: true
	property bool showBackButton: true
	property string title: "Library"
	property string library_uuid: ""

	function initLibraryModel(uuid, path){
        libraryModel.openLibrary(uuid, path)
	}
    LibraryModel{
        id: libraryModel
    }

	function backButtonPressed(){
		if (!libraryModel.prevFolder())
			stackView.pop();
	}

	function addButtonPressed(){
		newBooksDialog.open();
	}
	FileDialog{
		id: newBooksDialog
		title: "Select books"
		folder: StandardPaths.writableLocation(StandardPaths.DocumentsLocation)
		//selectMultiple: true
		fileMode: FileDialog.OpenFiles
		nameFilters: "Books (*.epub *pdf)"
		onAccepted: {
			console.log(newBooksDialog.file)

			LibraryModel.addBooks(newBooksDialog.files)
		}
	}
	GridView{
        property int coverWidth : libraryModel.coverWidthProp
		id: libraryGrid
		Layout.fillWidth: true
		Layout.fillHeight: true
        model: libraryModel
		property int elementsPerRow: (width / coverWidth)
		cellWidth: coverWidth + ((width % (coverWidth + 0.0)) / (elementsPerRow))
		cellHeight: coverWidth * 1.6 + 40
		clip:true
		ScrollBar.vertical: ScrollBar{}
		delegate: LibraryViewDelegate{
		    coverWidth: libraryGrid.coverWidth
		    coverHeight: libraryGrid.coverWidth * 1.6

		}

	}
}
