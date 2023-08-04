import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Controls.Material 2.14
import "pages"

ApplicationWindow {
    id: root
    visible: true
    minimumWidth: 480
    minimumHeight: 640
    title: qsTr("Hello World")

    Material.theme: Material.Light
    Material.primary: Material.Indigo
    Material.accent: Material.Orange


    // NavBar
    header: ToolBar {
        id: header
        ///color: Material.primary
        height: 72

        Label {
            anchors.fill: parent
            text: qsTr("Hello World")
            font.pixelSize: 30
            horizontalAlignment: Text.AlignHLeft
            verticalAlignment: Text.AlignVCenter
            leftPadding: 16
        }
    }

    StackView {
        id: stackView
        anchors.top: header.bottom
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.bottom: parent.bottom

        initialItem: greetingView
    }

    Component {
        id: greetingView

        Greeting {
            hasNext: true
            onNext: {
                stackView.push(mcVersionView)
            }
        }
    }

    Component {
        id: mcVersionView

        MCVersion {
            hasPrevious: true
            hasNext: true
            onPrevious: {
                stackView.pop()
            }
            onNext: {
                stackView.push(modloaderView)
            }
        }
    }

    Component {
        id: modloaderView

        ModLoader {
            hasPrevious: true
            hasNext: true

            onPrevious: {
                stackView.pop()
            }

            onNext: {
                //stackView.push(modsView)
            }
        }
    }
}