extern crate qml;
use qml::*;

fn main() {
    let mut engine = QmlEngine::new();

#[cfg(debug_assertions)]
    engine.load_file("src/qml/client_gui.qml");
#[cfg(not(debug_assertions))]
    engine.load_data(include_str!("client_gui.qml"));
    engine.exec();
}
