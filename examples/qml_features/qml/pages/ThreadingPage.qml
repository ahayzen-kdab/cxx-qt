// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

import com.kdab.cxx_qt.demo 1.0

Item {
    ThreadingWebsite {
        id: website
    }

    ColumnLayout {
        anchors.centerIn: parent

        Text {
            Layout.alignment: Qt.AlignHCenter
            text: "Url: " + website.url
        }

        Text {
            Layout.alignment: Qt.AlignHCenter
            text: "Title: " + website.title
        }

        Button {
            Layout.alignment: Qt.AlignHCenter
            text: "Change Url"

            onClicked: website.changeUrl()
        }

        Button {
            Layout.alignment: Qt.AlignHCenter
            text: "Fetch Title"

            onClicked: website.fetchTitle()
        }
    }
}
