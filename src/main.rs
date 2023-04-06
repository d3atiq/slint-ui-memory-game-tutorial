
fn main() {
    MainWindow::new().unwrap().run().unwrap();
    
    // match MainWindow::new() {
    //     Ok(window) => window.run(),
    //     Err(error) => println!("Error creating window: {error:?}"),
    // }
}

slint::slint! {
    export component MainWindow inherits Window {
        Text {
            text: "hello world";
            color: green;
        }
    }
}
