import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Controls.Material 2.14

Item {
    property var hasNext: false
    property var hasPrevious: false
    property var hasFinal: false

    signal next()
    signal previous()
    default property alias content: column.children

    Column {
        id: column
        anchors.fill: parent
        anchors.bottom: parent.bottom
        anchors.margins: 16
    }

    Rectangle {
        anchors.bottom: parent.bottom
        anchors.left: parent.left
        anchors.right: parent.right
        height: 64
        color: Material.color(Material.Grey, Material.Shade300)

        Item {
            anchors.fill: parent
            anchors.margins: 16

            Button {
                visible: hasPrevious
                anchors.left: parent.left
                anchors.verticalCenter: parent.verticalCenter

                highlighted: true
                text: qsTr("Previous")
                onClicked: previous()
            }

            Button {
                visible: hasNext
                anchors.right: parent.right
                anchors.verticalCenter: parent.verticalCenter

                highlighted: true
                text: qsTr("Next")
                onClicked: next()
            }

            Button {
                visible: hasFinal
                anchors.right: parent.right
                anchors.verticalCenter: parent.verticalCenter

                highlighted: true
                text: qsTr("Finish")
                onClicked: next()
            }
        }
        
    }
}