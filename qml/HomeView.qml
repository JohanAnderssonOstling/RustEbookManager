import QtQuick 2.9
import QtQuick.Controls 2.5
import QtQuick.Layouts 1.3
import QtQuick.Dialogs


ColumnLayout {

	property bool showAddButton: true
	property bool showBackButton: false

	property string title: "Home"
	function addButtonPressed(){
		createLibraryDialog.open();
		homeGrid.focus = true
	}

	FolderDialog{
    	id: createLibraryDialog
    	title: "Select folder"
    	currentFolder: StandardPaths.writableLocation(StandardPaths.DocumentsLocation)
    	onAccepted: {
    		console.log(createLibraryDialog.selectedFolder)

    		HomeModel.addLibrary(createLibraryDialog.selectedFolder)
        }
    }

	GridView {
		id: homeGrid
		Layout.fillWidth: true
		Layout.fillHeight: true
		cellWidth: 260
		cellHeight: 330
		focus: true
		clip: true
		activeFocusOnTab: true
		model: HomeModel
		highlight: Rectangle { color: "lightblue" }
		delegate:
			HomeViewDelegate{

			}

		Keys.onDeletePressed:{
			HomeModel.deleteLibrary(homeGrid.currentIndex)
		}
		Keys.onReturnPressed: {
			LibraryModel.setLibrary(HomeModel.getLibrary(homeGrid.currentIndex))
			stackView.push(libraryView)
		}
	}
	Text{
		text: homeGrid.focus ? "focus" : "not focus"
	}

}


