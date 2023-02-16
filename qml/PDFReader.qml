import QtQuick
import QtQuick.Pdf
Rectangle{
    property bool showAddButton: false
    	property bool showBackButton: true
    	property string title: "PDF Viewer"
    	function backButtonPressed(){
    	    stackView.pop();
    	}
    property url documentSource
    anchors.fill: parent
    color: "red"
    id :rect
    focus: true
    PdfPageView {
        id : pdfPageView
        document: PdfDocument { source: documentSource }
        renderScale: 1.5
        Keys.onPressed: {
            console.log("Pressed key: " + event.key + " Right: " + Qt.Key_Right + " Left: " + Qt.Key_Left)
            if (event.key == Qt.Key_Right){
                pdfPageView.goToPage(pdfPageView.currentPage + 1);
            }
            else if(event.key == Qt.Key_Left){
                pdfPageView.goToPage(pdfPageView.currentPage - 1);
            }
        }

        MouseArea{
            anchors.fill: parent
            onClicked: (mouse) => {
                pdfPageView.scaleToPage(rect.width, rect.height);
                pdfPageView.focus = true;
            }
        }
    }
}
