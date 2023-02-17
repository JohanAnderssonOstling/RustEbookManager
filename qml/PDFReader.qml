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
    anchors.fill: parent
    id :rect
    focus: true
    function init(loc){
        console.log("Setting book page to " + loc);
        pdfPageView.goToPage(loc);
    }

    property var pdfdocument

    PdfPageView {
        id : pdfPageView
        document: PdfDocument{
            id: pdfDocument
            source: documentSource
            onStatusChanged: {
                if(status == PdfDocument.Ready){
                    console.log("Pdf document is ready: " +	 "init_loc is" + init_read_location);
                }
                //rect.init(init_read_location);
            }
        }
        function setPage(){
        }
        function changePage(delta){
            console.log("Change page: " + delta)
            let pageCount = pdfPageView.document.pageCount;
            var newPage = pdfPageView.currentPage + delta;
            if(newPage < 0 || newPage >= pageCount){
                console.log("Page out of range: " + newPage)
                return;
            }
            console.log("Setting page to: " + newPage);
            var read_percentage = Math.round((newPage / pageCount) * 100);
            pdfPageView.goToPage(newPage);
            libraryModel.setBookLocation(uuid, newPage.toString(), read_percentage);
        }
        function nextPage(){
            pdfPageView.changePage(1);
            console.log("test");
        }
        function previousPage(){
            pdfPageView.changePage(-1);
        }

        renderScale: 1.5
        Layout.fillWidth: true
        Layout.fillHeight: true
        property bool first: false
        onStatusChanged: {

            if (status == 1 && !first){
                first = true;
                console.log("Pdf page is ready" + status + " init_loc is" + init_read_location);

                //pdfPageView.changePage(init_read_location);
                pdfPageView.goToPage(init_read_location);
            }
            //}

        }

        Keys.onPressed: {
            console.log("Pressed key: " + event.key + " Right: " + Qt.Key_Right + " Left: " + Qt.Key_Left)
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
