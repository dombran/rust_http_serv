
use gtk::glib;


use gtk::prelude::*;

//use gtksys:;

//use gtk;
use gtk::{ ApplicationWindow, Builder, Button, MessageDialog};


fn on_activate(application: &gtk::Application) {
    //let glade_src = "Unsaved_1.glade"; 
    //let builder = gtk::Builder::new();
    //let err = builder.add_from_file(glade_src);
    //let window: ApplicationWindow = builder.application().set_application(application);
    //window.set_application(Some(application));
    
    
    let glade_src = include_str!("Unsaved_2.glade"); 
    //let glade_src = include_str!("builder_basics.ui");
    let builder = Builder::from_string(glade_src);
    let window: ApplicationWindow = builder.object("window1").expect("Couldn't get window1");
    window.set_application(Some(application));
    
    //let bigbutton: Button = builder.object("button1").expect("Couldn't get button1");
    //let dialog: MessageDialog = builder
    //    .object("messagedialog1")
    //    .expect("Couldn't get messagedialog1");
    //dialog.connect_delete_event(|dialog, _| {
    //    dialog.hide();
    //    gtk::Inhibit(true)
    //});
    //bigbutton.connect_clicked(glib::clone!(@weak dialog => move |_| dialog.show_all()));
    
    window.show_all(); 

}

fn main() { 
    
    
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.builder_basics"),
        Default::default(),
    );

    application.connect_activate(on_activate);//build_ui);

    application.run();
}

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("builder_basics.ui");
    let builder = Builder::from_string(glade_src);

    let window: ApplicationWindow = builder.object("window1").expect("Couldn't get window1");
    window.set_application(Some(application));
    let bigbutton: Button = builder.object("button1").expect("Couldn't get button1");
    let dialog: MessageDialog = builder
        .object("messagedialog1")
        .expect("Couldn't get messagedialog1");

    dialog.connect_delete_event(|dialog, _| {
        dialog.hide();
        gtk::Inhibit(true)
    });

    bigbutton.connect_clicked(glib::clone!(@weak dialog => move |_| dialog.show_all()));
    window.show_all(); 
}

