use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{
    Align, Application, ApplicationWindow, Button, Entry, Label, Orientation, Spinner, TextBuffer,
    TextView,
};
const APP_ID: &str = "com.bubelov.thunder";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let url_label = Label::builder().label("Server URL").build();
    let url = Entry::builder()
        .placeholder_text("mynode.com")
        .margin_top(8)
        .build();
    let server_pem_label = Label::builder().label("Server PEM").margin_top(16).build();
    let server_pem_buffer = TextBuffer::builder().text("\n").build();
    let server_pem = TextView::builder()
        .buffer(&server_pem_buffer)
        .margin_top(8)
        .build();
    let client_pem_label = Label::builder().label("Client PEM").margin_top(16).build();
    let client_pem_buffer = TextBuffer::builder().text("\n").build();
    let client_pem = TextView::builder()
        .buffer(&client_pem_buffer)
        .margin_top(8)
        .build();
    let client_key_pem_label = Label::builder()
        .label("Client key PEM")
        .margin_top(16)
        .build();
    let client_key_pem_buffer = TextBuffer::builder().text("\n").build();
    let client_key_pem = TextView::builder()
        .buffer(&client_key_pem_buffer)
        .margin_top(8)
        .build();
    let connect = Button::builder().label("Connect").margin_top(16).build();
    let spinner = Spinner::builder().valign(Align::Fill).build();

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(16)
        .margin_bottom(16)
        .margin_start(16)
        .margin_end(16)
        .build();

    gtk_box.append(&url_label);
    gtk_box.append(&url);
    gtk_box.append(&server_pem_label);
    gtk_box.append(&server_pem);
    gtk_box.append(&client_pem_label);
    gtk_box.append(&client_pem);
    gtk_box.append(&client_key_pem_label);
    gtk_box.append(&client_key_pem);
    gtk_box.append(&connect);

    let center_box = gtk::CenterBox::builder()
        .orientation(Orientation::Vertical)
        .build();
    center_box.set_start_widget(Some(&gtk_box));

    connect.connect_clicked(clone!(@strong center_box, @strong spinner =>
        move |_| {
        center_box.set_start_widget(gtk::Widget::NONE);
        center_box.set_center_widget(Some(&spinner));
        spinner.set_spinning(true);
    }));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Thunder")
        .child(&center_box)
        .default_width(400)
        .default_height(600)
        .build();

    window.present();
}
