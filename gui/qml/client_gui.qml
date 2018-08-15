import QtQuick 2.0
imprt QtQick.Controls 1.0

ApplicationWindow {
    visible: true
    TextInput {
        id: cl_input
        x: 8
        y: 452
        width: 624
        height: 20
        text: qsTr("Text Input")
        font.pixelSize: 12
    }

    Text {
        id: cl_output
        x: 8
        y: 8
        width: 624
        height: 438
        text: qsTr("Text")
        font.pixelSize: 12
    }

}
