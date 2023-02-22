
import QtQuick
import QtQuick.Controls
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Pdf
TreeView{
    id: tocTreeView
    implicitWidth: parent.width
    implicitHeight: parent.height
    columnWidthProvider: {return width}
    model: PdfBookmarkModel{
        document: pdfDocument
    }
}
