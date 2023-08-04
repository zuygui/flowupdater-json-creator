import QtQuick 2
import QtQuick.Controls 2
import QtQuick.Controls.Material 2
import QtQuick.Layouts 1
import MinecraftVersions 1

PageLayout {
    ColumnLayout {
        width: parent.width
        spacing: 20

        MinecraftVersions {
            id: minecraftVersions
        }

        Component.onCompleted: {
            minecraftVersions.getMinecraftVersions()
        }

        GroupBox {
            label: Text {
                text: qsTr("Which Minecraft version would you use?")
            }
            Layout.fillWidth: true

            ComboBox {
                id: modloaderSelection
                model: minecraftVersions
                textRole: "version"
                currentIndex: 0
            }
        }
    }
}