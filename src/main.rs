use gtk::prelude::*;
use glib::clone;
use clap::Clap;

fn command(command: &str){
    let split: Vec<&str> = command.split(' ').collect();
    let mut cmd = std::process::Command::new(split[0]);
    for arg in &split[1..] {
        cmd.arg(arg);
    }
    cmd.spawn().expect("POWERMENU COMMAND FAILED");
}

#[derive(Clap, Clone)]
#[clap(version = "1.0", author = "Beinsezii")]
struct Opts {
    #[clap(long, default_value="xset dpms force off")]
    lock: String,

    #[clap(long, default_value="systemctl suspend")]
    suspend: String,

    #[clap(long, default_value="i3-msg exit")]
    logout: String,

    #[clap(long, default_value="systemctl reboot")]
    reboot: String,

    #[clap(long, default_value="systemctl poweroff")]
    poweroff: String,

    #[clap(long, default_value="100")]
    button_size: i32,
}

fn main() {
    gtk::init().expect("Gtk init failed");
    let opts = Opts::parse();

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_type_hint(gdk::WindowTypeHint::Dialog);

    let buts_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    buts_box.set_homogeneous(true);

    let buts = [
        gtk::Button::with_label("Cancel"),
        gtk::Button::with_label("Lock"),
        gtk::Button::with_label("Suspend"),
        gtk::Button::with_label("Log Out"),
        gtk::Button::with_label("Reboot"),
        gtk::Button::with_label("Power Off"),
    ];
    let cmd_opts = [opts.lock, opts.suspend, opts.logout, opts.reboot, opts.poweroff];
    for x in 0..cmd_opts.len() {
        let arg = cmd_opts[x].clone();
        buts[x+1].connect_clicked(move |_| command(&arg));
    }
    for but in buts.iter() {
        but.connect_clicked(clone!(@weak window => move |_| unsafe{window.destroy()}));
        but.set_size_request(opts.button_size, opts.button_size);
        buts_box.pack_start(but, true, true, 0);
    }

    window.add(&buts_box);
    window.show_all();
    window.connect_destroy(|_| gtk::main_quit());
    gtk::main();
}
