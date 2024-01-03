use gtk;
use tray_item::TrayItem;

use hotkey;
use std::process::Command;
use std::thread;

fn main() {
    thread::spawn(move || {
        register_hk();
    });

    gtk::init().unwrap();

    let mut tray = TrayItem::new("Dell Switcher", "display").unwrap();
    tray.add_label("Dell Switcher").unwrap();

    tray.add_menu_item("Switch to Type-C", || {
        do_switch();
    })
    .unwrap();

    tray.add_menu_item("Switch to Type-C", || {
        brigtness_up();
    })
    .unwrap();

    tray.add_menu_item("Switch to Type-C", || {
        brigtness_down();
    })
    .unwrap();

    tray.add_menu_item("Quit", || {
        gtk::main_quit();
    })
    .unwrap();

    gtk::main();
}

fn register_hk() {
    let mut hk = hotkey::Listener::new();
    hk.register_hotkey(hotkey::modifiers::ALT, hotkey::keys::END, || do_switch())
        .unwrap();

    hk.listen();
}

fn notify(title: &str, message: &str) {
    let mut command = Command::new("notify-send");
    command.arg(title);
    command.arg(message);
    command.output().unwrap();
}

fn brigtness_up() {
    let output = Command::new("ddcutil")
        .arg("--bus=5")
        .arg("setvcp")
        .arg("60")
        .arg("0x19")
        .output()
}

fn do_switch() {
    // notify("DELL Switcher", "Switch to Type-C");


    let output = Command::new("ddcutil")
        .arg("--bus=5")
        .arg("setvcp")
        .arg("60")
        .arg("0x19")
        .output()
        .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
