import QtQuick 2.12
import QtQuick.Window 2.12
import com.kdab.energy 1.0
import QtGraphicalEffects 1.15



BorderImage {
    id: panel
    source: "./images/panel.png"
    width: 900
    clip: true
    height: parent.height-30
    anchors.verticalCenter: parent.verticalCenter
    border.left: 0; border.top: 25
    border.right: 25; border.bottom: 25
    Text {
        font.family: "Open Sans"
        font.italic: true
        font.pixelSize: 45
        color: "#a9deff"
        font.weight: Font.Light
        text:"Rust Sensors KDAB beach house Demo"
        x: kdabL.x
        y:20
    }

    Image {
        id: kdabL
        source: "./images/kdabLogo.png"
        x: -parent.x/1.5 + 60
        y: 120

    }
    Text {
        font.family: "Open Sans"
        horizontalAlignment: Text.AlignJustify
        width: 520
        wrapMode: Text.Wrap
        font.pixelSize: 16
        font.weight: Font.Light
        color: "#a9deff"
        text:"<b>The KDAB Group is the leading provider for Qt, C++ and 3D/OpenGL software expertise across desktop, embedded and mobile platforms.</b>
<p>
The KDAB Group is the leading software consulting, development and training provider for Qt, C++ and 3D/OpenGL. Since 1999, KDAB’s unique software expertise has helped thousands of customers to improve quality, productivity and time to market across desktop, embedded and mobile platforms. Our customers – several from the Fortune 500 list – are active within Automotive, Biotech, Medical, Industrial Embedded, Entertainment, Government, Consumer Software and other innovative sectors. We take great pride in delivering our projects successfully, on time and with high quality."
        anchors.left: kdabL.right
        anchors.leftMargin: 25
        y: kdabL.y - 10
    }
    Image {
        id: rust
        source: "./images/RLogolarge.png"
        anchors.horizontalCenter: kdabL.horizontalCenter
        anchors.top: kdabL.bottom
        anchors.topMargin: 130
    }
    Text {
        font.family: "Open Sans"
        width: 520
        wrapMode: Text.Wrap
        horizontalAlignment: Text.AlignJustify
        font.pixelSize: 16
        font.weight: Font.Light
        color: "#a9deff"
        text:"<b>Rust</b><p>
Blazingly fast and memory-efficient: with no runtime or garbage collector, it can power performance-critical services, run on embedded devices, and easily integrate with other languages.
<p>
Rust’s rich type system and ownership model guarantee memory-safety and thread-safety — enabling you to eliminate many classes of bugs at compile-time.
<p>
Rust has great documentation, a friendly compiler with useful error messages, and top-notch tooling — an integrated package manager and build tool, smart multi-editor support with auto-completion and type inspections, an auto-formatter, and more."
        anchors.left: kdabL.right
        anchors.leftMargin: 25
        y: rust.y - 10
    }
    MouseArea {
        anchors.fill: parent
        onClicked: about = false
    }
}