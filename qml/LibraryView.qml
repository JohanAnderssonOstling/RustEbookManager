import QtQuick 2.0
import QtQuick.Controls 2.5
import QtQuick.Layouts 1.3
import Qt.labs.platform 1.1


ColumnLayout{
	property bool showAddButton: true
	property bool showBackButton: true
	property int coverWidth: LibraryModel.coverWidth
	property string title: "Library"
	function backButtonPressed(){
		if (!LibraryModel.prevFolder())
			stackView.pop();
	}
	function componentClicked(model){
		if (LibraryModel.isFolder(model.index)){
			LibraryModel.changeFolder(model.id);
		}
		else{
			var path = LibraryModel.getFullPath(model.index);
			epubReader.row = model.index;
			epubReader.title = model.name
			epubReader.setBookUrl(path, model.location);
			stackView.push(epubReader);
		}
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
		id: libraryGrid
		Layout.fillWidth: true
		Layout.fillHeight: true

		property int elementsPerRow: (width / coverWidth)
		cellWidth: coverWidth + ((width % (coverWidth + 0.0)) / (elementsPerRow))
		cellHeight: coverWidth * 1.6 + 40
		clip:true
		model: LibraryModel
		focus: true
		ScrollBar.vertical: ScrollBar{}
		delegate:Item{
			Column{
			x: (libraryGrid.cellWidth - coverWidth)/ 2
			width: coverWidth
			height: parent.cellHeight

			Rectangle{
				id: coverPlaceHolder
				visible: !model.hasCover

				width: coverWidth
				height: coverWidth * 1.2
				color: "cornflowerblue"
				MouseArea{
					anchors.fill: parent
					onClicked: componentClicked(model)
				}
					Text{
						color: "white"
						anchors.fill: parent
						anchors.horizontalCenter: parent.horizontalCenter
						elide: Label.ElideRight
						wrapMode: Text.Wrap
						text: model.name

				}
			}
			Image{

				id: cover
				visible: model.hasCover
				source: "file://" + model.cover
				asynchronous: true
				Rectangle{
					anchors.fill: parent
					color:"transparent"
					border.width: 1
					border.color: "lightgray"
				}
				MouseArea{
					anchors.fill: parent
					onClicked: componentClicked(model)
				}
			}


			Text{
				width: coverWidth
				visible: model.hasCover
				text:model.name
				horizontalAlignment: Text.AlignHCenter

				elide: Text.ElideRight
			}
			Text{
				width: coverWidth
				visible: model.hasCover
				text: model.author
				horizontalAlignment: Text.AlignHCenter
			}
		}
	}}
}
