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
        function setPage(){
        }
        function changePage(delta){
            let pageCount = pdfPageView.document.pageCount;
            var newPage = pdfPageView.currentPage + delta;
            if(newPage < 0 || newPage >= pageCount){
                console.log("Page out of range: " + newPage)
                return;
            }
            var read_percentage = Math.round((newPage / pageCount) * 100);
            pdfPageView.goToPage(newPage);
            libraryModel.setBookLocation(uuid, newPage.toString(), read_percentage);
        }
        function nextPage(){pdfPageView.changePage(1);}
        function previousPage(){pdfPageView.changePage(-1);}
        Layout.alignment: Qt.AlignBottom
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

                pdfPageView.goToPage(init_read_location);
            }
        }
        focus: true
            Keys.onPressed: (event) => {
                focus = true;
                    if (event.key == Qt.Key_Right){
                            pdfPageView.nextPage();
                        }
                        else if(event.key == Qt.Key_Left){
                            pdfPageView.previousPage()
                        }
                    }

        MouseArea{
            anchors.fill: parent
            onClicked: (mouse) => {
                           //Next page if clicked on right third of area mousarea
                           if(mouseX > width * 0.66){
                               pdfPageView.nextPage();
                           }
                           else if(mouseY < width * 0.33){
                               pdfPageView.previousPage()
                           }
                       }
        }
    }
    Text{
        text: pdfPageView.currentPage
    }
}
