import QtQuick 2
import QtQuick.Controls 2
import QtQuick.Controls.Material 2
import QtQuick.Layouts 1

PageLayout {
    ColumnLayout {
        width: parent.width
        spacing: 20

        GroupBox {
            label: Text {
                text: qsTr("Do you want to use a modloader?")
            }
            Layout.fillWidth: true

            ColumnLayout {
                id: modLoaderChoice

                RadioButton {
                    id: noModloader
                    text: qsTr("No")
                }

                RadioButton {
                    id: yesModloader
                    text: qsTr("Yes")
                    checked: true
                }
            }
        }

        GroupBox {
            id: modLoaderGroupBox
            label: Text {
                text: qsTr("Which modloader do you want to use?")
                color: yesModloader.checked ? Material.color(Material.Grey, Material.Shade900) : Material.color(Material.Grey, Material.Shade400)
            }
            Layout.fillWidth: true
            enabled: yesModloader.checked

            ComboBox {
                id: modloaderSelection
                model: ListModel {
                    ListElement { text: "Forge" }
                    ListElement { text: "Fabric" }
                }
            }
        }
    }
}