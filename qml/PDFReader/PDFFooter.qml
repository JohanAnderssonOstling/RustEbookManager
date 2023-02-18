
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
ToolBar {

    id: pdfFooterToolBar
    Layout.alignment: Qt.AlignBottom
    Layout.fillWidth: true
    RowLayout{
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

}
