// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick.Window 2.12

Window {
    height: 480
    title: qsTr("CXX-Qt: QML Features")
    visible: true
    width: 640

    RowLayout {
        anchors.fill: parent

        ListView {
            id: view
            Layout.fillHeight: true
            Layout.preferredWidth: 150

<<<<<<< HEAD
    ThreadingWebsite {
        id: website
    }

    Column {
        anchors.fill: parent
        anchors.margins: 10
        spacing: 10
=======
            currentIndex: 0
            delegate: ItemDelegate {
                highlighted: ListView.isCurrentItem
                text: model.name
                width: ListView.view.width

                readonly property string source: model.source
>>>>>>> 58ee7afc (WIP: qml_features: create pages for each example)

                onClicked: ListView.view.currentIndex = index
            }
            model: ListModel {
                ListElement {
                    name: "Base Class"
                    source: "qrc:/pages/BaseClassPage.qml"
                }
                ListElement {
                    name: "Invokables"
                    source: "qrc:/pages/InvokablesPage.qml"
                }
                ListElement {
                    name: "Properties"
                    source: "qrc:/pages/PropertiesPage.qml"
                }
                ListElement {
                    name: "Serialisation"
                    source: "qrc:/pages/SerialisationPage.qml"
                }
                ListElement {
                    name: "Signals"
                    source: "qrc:/pages/SignalsPage.qml"
                }
                ListElement {
                    name: "Threading"
                    source: "qrc:/pages/ThreadingPage.qml"
                }
                ListElement {
                    name: "Types"
                    source: "qrc:/pages/TypesPage.qml"
                }
            }
        }

        Loader {
            id: content
            Layout.fillHeight: true
            Layout.fillWidth: true
            source: view.currentItem.source
        }
    }

    // Serialisation {
    //     id: myData
    //     number: myObject.number
    //     string: myObject.string
    // }

    // MyObject {
    //     id: myObject
    //     number: 1
    //     string: "My String " + myObject.number
    // }

    // Column {
    //     anchors.fill: parent
    //     anchors.margins: 10
    //     spacing: 10

    //     Label {
    //         text: "Number: " + myObject.number
    //     }

    //     Label {
    //         text: "String: " + myObject.string
    //     }

    //     Button {
    //         text: "Increment Number"

    //         onClicked: myObject.number = myObject.incrementNumber(myObject.number)
    //     }

    //     Button {
    //         text: "Increment Number (self)"

    //         onClicked: myObject.incrementNumberSelf()
    //     }

    //     Button {
    //         text: "Print Data"

    //         onClicked: console.warn(myData.asJsonStr())
    //     }

    //     Text {
    //         text: "Url: " + website.url
    //     }

    //     Text {
    //         text: "Title: " + website.title
    //     }

    //     Button {
    //         text: "Change Url"

    //         onClicked: website.changeUrl()
    //     }

    //     Button {
    //         text: "Fetch Title"

    //         onClicked: website.fetchTitle()
    //     }
    // }

    // Component.onCompleted: myObject.sayHi(myObject.string, myObject.number)
}
