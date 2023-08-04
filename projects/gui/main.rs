use qmetaobject::{prelude::*, QUrl, QQuickStyle};

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod gui;
mod minecraft;

qrc!(root_qml,
    "" {
        "ui/main.qml" as "main.qml",
        "ui/pages/PageLayout.qml" as "pages/PageLayout.qml",
        "ui/pages/Greeting.qml" as "pages/Greeting.qml",
        "ui/pages/MCVersion.qml" as "pages/MCVersion.qml",
        "ui/pages/ModLoader.qml" as "pages/ModLoader.qml",
    }
);

fn main() {
    QQuickStyle::set_style("Material");
    qmetaobject::log::init_qt_to_rust();
    pretty_env_logger::init();

    info!("Starting the application");

    // Register types for QML
    gui::register_types();


    // Create a QML engine
    let mut engine = QmlEngine::new();

    // Load resources
    root_qml();

    engine.load_url(QUrl::from(QString::from("qrc:/main.qml")));
    engine.exec();
}
