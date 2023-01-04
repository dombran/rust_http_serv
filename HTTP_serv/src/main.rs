
use gtk::glib;


use gtk::prelude::*;

mod server;

//use gtksys:;

//use gtk;
use gtk::{ ApplicationWindow, Builder, Button, MessageDialog};

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};



fn main() { 
    
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = server::ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    
    
    
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.builder_basics"),
        Default::default(),
    );

    application.connect_activate(on_activate);//build_ui);

    application.run();
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
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

