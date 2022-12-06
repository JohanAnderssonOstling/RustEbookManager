import QtQuick 2.4
import QtQuick.Controls 2.5
import QtQuick.Layouts 1.3
GridLayout{
    rows: 4
    columns: 3
    Layout.fillWidth: true
    Layout.fillHeight: true
    Label{
        text: qsTr("Library name:")
    }
    TextEdit{
        Layout.fillWidth: true
    }
    Button{
        text: "file"
    }
    Rectangle{
        Layout.fillWidth: true
    }
    Button{
        text: qsTr("Create")

    }
    Button{
        text: qsTr("Cancel")
    }
}
