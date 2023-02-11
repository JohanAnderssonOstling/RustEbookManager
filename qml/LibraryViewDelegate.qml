import QtQuick 2.0
import QtQuick.Controls 2.5
import QtQuick.Layouts 1.3
import Qt.labs.platform 1.1
import johandost.LibraryModel 1.0



Column{
    id: column
    property int coverWidth
    property int coverHeight
    width: parent.cellWidth
    height: parent.cellHeight


	Rectangle{
		id: coverPlaceHolder
		visible: !hasCover
        width: column.coverWidth
        height: column.coverHeight
		color: "cornflowerblue"
		MouseArea{
		    anchors.fill: parent
            onClicked:{
                libraryGrid.model.changeFolder(id)
            }
        }

    }
	Image{
        id: image
         visible: hasCover
        source: "file://" + cover
        asynchronous: true
        MouseArea{
            anchors.fill: parent
            onClicked:{
               var epubReaderComponent = Qt.createComponent("EpubReader.qml");
               if (epubReaderComponent.status == Component.Ready){
                   var epubReader = epubReaderComponent.createObject(parent);
                   epubReader.readerUrl = "file://" + "/home/johandost/CLionProjects/RustEbookManager/web/epubreader.html";
                   epubReader.bookUrl = "file://" + "/home/johandost/Hem/Documents/Böcker/Star Trek - Abramsverse/Star Trek_ Abramsverse - 02 - Into Darkness - Alan Dean Foster.epub";
                   console.log("Pushing epubReader");
                   stackView.push(epubReader);
               }
               else
                   console.log("error loading component");

            }
        }

}



	Text{
	width: parent.width
	height: 40
    	text:name
    	horizontalAlignment: Text.AlignHCenter
        wrapMode: Text.Wrap
        elide: Text.ElideRight
    }


}