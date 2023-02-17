import QtQuick 2.0
import QtQuick.Controls 2.5
import QtQuick.Layouts 1.3
import Qt.labs.platform 1.1
import johandost.LibraryModel 1.0
import QtQuick.Pdf
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
                var pdfReaderComponent = Qt.createComponent("PDFReader.qml");
                if (pdfReaderComponent.status == Component.Ready){
                    var pdfReader = pdfReaderComponent.createObject(parent);
                    pdfReader.documentSource = "file://" + path
                    pdfReader.title = name +" uuid: " +uuid + " location: " + location
                    pdfReader.uuid = uuid
                    pdfReader.init_read_location = location
                    pdfReader.init(location);
                    //pdfReader.model = model
                    stackView.push(pdfReader);
                }
                else{
                    console.log("error loading component");
                    console.log(pdfReaderComponent.errorString());
                }

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

                console.log("clicked")
                var pdfReaderComponent = Qt.createComponent("PDFReader.qml");
                if (pdfReaderComponent.status == Component.Ready){
                    var pdfReader = pdfReaderComponent.createObject(parent);
                    pdfReader.documentSource = "file://" + path
                    pdfReader.title = name +" uuid: " +uuid + " location: " + location
                    pdfReader.uuid = uuid
                    pdfReader.init_read_location = location
                    //pdfReader.init(location);
                    //pdfReader.model = model
                    stackView.push(pdfReader);
                }
                else{
                    console.log("error loading component");
                    console.log(pdfReaderComponent.errorString());
                }
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
