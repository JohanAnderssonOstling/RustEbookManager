import QtQuick
import QtQuick.Pdf
import QtQuick.Layouts
ColumnLayout{
    property bool showAddButton: false
    property bool showBackButton: true
    function backButtonPressed(){
        stackView.pop();
    }
    property var title
    property url documentSource
    property string uuid
    property var init_read_location

    PdfPageView {
        id : pdfPageView
        document: PdfDocument{
            id: pdfDocument
            source: documentSource
        }
        property var pageCount: pdfPageView.document.pageCount
        function setPage(newPage){
            let pageCount = pdfPageView.document.pageCount;
            pdfPageView.goToPage(newPage);
            if(newPage < 0 || newPage >= pageCount){
                console.log("Page out of range: " + newPage)
                return;
            }
            var read_percentage = Math.round((newPage / pageCount) * 100);
            pdfPageView.setPage(newPage);
            libraryModel.setBookLocation(uuid, newPage.toString(), read_percentage);
        }
        function changePage(delta){
            var newPage = pdfPageView.currentPage + delta;
            pdfPageView.setPage(newPage);
        }

        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
        Layout.fillWidth: true
        Layout.fillHeight: true
        Layout.margins: 20
        property bool first: false
        border.color: "black"
        border.width: 20
        onStatusChanged: {
            pdfPageView.focus = true;
            focus = true;
            scaleToPage(parent.width, parent.height);
            if (status == 1 && !first){
                first = true;
                console.log("Loaded book: " + name + " at position " + init_read_location);
                window.title = pdfDocument.subject
                pdfPageView.goToPage(init_read_location);
            }
        }
        focus: true
            Keys.onPressed: (event) => {
                focus = true;
                    if (event.key == Qt.Key_Right){
                            pdfPageView.changePage(1);
                        }
                        else if(event.key == Qt.Key_Left){
                            pdfPageView.changePage(-1);
                        }
                    }


    }
    //Button{

       //text: "Menu"
        //onClicked: {
          //  rootDrawer.open();
        //}
      //}
}
