import QtQuick 2.9
import QtQuick.Controls 2.5
import QtQuick.Layouts 1.3

Column{
    Rectangle{
		anchors.horizontalCenter: parent.horizontalCenter
		width: 200
		height: 300
		color: "lightblue"
		MouseArea{
			anchors.fill: parent
			acceptedButtons: Qt.LeftButton | Qt.RightButton
			onClicked: {
				if (mouse.button & Qt.LeftButton){
					var libraryViewComponent = Qt.createComponent("LibraryView.qml", {library_uuid: model.uuid});
					if (libraryViewComponent.status == Component.Ready){
                        var libraryView = libraryViewComponent.createObject(parent);
                        libraryView.initLibraryModel(model.uuid, model.path);
                    }
					stackView.push(libraryView);
				}
				else if(mouse.button & Qt.RightButton){
				    HomeModel.deleteLibrary(model.uuid);
			}}
		}
	}
	Label{
		anchors.horizontalCenter: parent.horizontalCenter
		elide: Text.ElideRight
		text: name
	}
}