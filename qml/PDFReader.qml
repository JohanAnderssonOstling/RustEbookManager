import QtQuick
import QtQuick.Pdf
Rectangle{
    property bool showAddButton: false
    	property bool showBackButton: true
    	property string title: "PDF Viewer"
    	function backButtonPressed(){
    	    stackView.pop();
    	}
    property var title
    property url documentSource
    property string uuid
    property int init_read_location
    anchors.fill: parent
    color: "red"
    id :rect
    focus: true
    function init(init_read_location){
    console.log("Init to" + init_read_location);
        pdfPageView.goToPage(init_read_location);
    }
    PdfPageView {
        function changePage(delta){
        console.log("Change page: " + delta)
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
        function nextPage(){ pdfPageView.changePage(1);}
        function previousPage(){
            pdfPageView.changePage(-1);
        }

        id : pdfPageView
        document: PdfDocument { source: documentSource }
        renderScale: 1.5

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
}
