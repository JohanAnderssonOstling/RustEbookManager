
import QtQuick
import QtQuick.Controls

ToolBar {
    width: parent.width
    id: pdfFooterToolBar
    RowLayout{}
    Label {
        id: pdfFooterPageLabel
        text: "Page " + pdfPageView.currentPage + " of " + pdfPageView.pageCount
    }
    ToolButton {
        id: pdfFooterPreviousPageButton
        text: "Previous"
        onClicked: {
            pdfPageView.previousPage()
        }
    }
}
