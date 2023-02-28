fn main() {
    MainWindow::new().run();
}

slint::slint! {
    MainWindow := Window {
        Text {
            text: "hello world";
            color: green;
        }
    }
}