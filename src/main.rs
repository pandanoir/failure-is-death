extern crate gtk;
extern crate sourceview;

use std::process::Command;
use gtk::prelude::*;
use gtk::{Button, Label, Entry, Window, WindowType};


fn exec_echo(input: &String) -> std::process::ChildStdout {
    Command::new("echo")
        .arg(input)
        .stdout(std::process::Stdio::piped())
        .spawn()
        .ok()
        .unwrap().stdout.unwrap()
}
fn exec(command: &mut Command, input: &String) -> std::process::Output {
    command.stdin(exec_echo(&input))
        .output()
        .expect("failed to execute process.")
}
fn main(){
    let title = "Failure is Death";

    if gtk::init().is_err() {
        println!("Faild to initialize GTK");
        return;
    }

    // windowの作成
    let window = Window::new(WindowType::Toplevel);
    window.set_title(title);
    window.set_default_size(400, 800);  // 横×縦

    // 各widgetの作成
    // let code = gtk::TextView::new();
    let code = sourceview::View::new();
    let language_select = gtk::ComboBoxText::new();
    language_select.append(Some("sed"), "Sed");
    language_select.append(Some("awk"), "Awk");
    language_select.append(Some("ruby"), "Ruby");
    language_select.set_active_id(Some("sed"));
    let label1 = Label::new(Some("test case 1")); let test_case1 = Entry::new();
    let label2 = Label::new(Some("test case 2")); let test_case2 = Entry::new();
    let label3 = Label::new(Some("test case 3")); let test_case3 = Entry::new();
    let button = Button::new_with_label("exec");

    // windowへの配置
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
    // child: &P, expand: bool, fill: bool, padding: u32

    vbox.pack_start(&code, true, true, 3);

    vbox.pack_start(&language_select, false, true, 2);
    vbox.pack_start(&label1, false, true, 2); vbox.pack_start(&test_case1, false, true, 2);
    vbox.pack_start(&label2, false, true, 2); vbox.pack_start(&test_case2, false, true, 2);
    vbox.pack_start(&label3, false, true, 2); vbox.pack_start(&test_case3, false, true, 2);

    vbox.pack_start(&button, false, true, 2);
    window.add(&vbox);

    // 初期表示
    window.show_all();

    // eventの設定
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    {
        // let code = code.clone();
        button.connect_clicked(move |_| {
            let buf = code.get_buffer().unwrap();
            let begin = gtk::TextBuffer::get_start_iter(&buf);
            let end = gtk::TextBuffer::get_end_iter(&buf);
            let _command = gtk::TextBuffer::get_text(&buf, &begin, &end, true).unwrap().to_string();
            let selected_language = language_select.get_active_id().unwrap().to_string();
            let mut command = Command::new(&selected_language);
            let mut command = if &selected_language == "awk" {
                command.arg(_command)
            } else {
                command.arg("-e").arg(_command)
            };

            let input1 = test_case1.get_text().unwrap().to_string();
            let input2 = test_case2.get_text().unwrap().to_string();
            let input3 = test_case3.get_text().unwrap().to_string();


            let output1 = exec(&mut command, &input1);
            let output2 = exec(&mut command, &input2);
            let output3 = exec(&mut command, &input3);

            let message = format!("output1: {}\noutput2: {}\noutput3: {}",
                std::str::from_utf8(&output1.stdout).unwrap(),
                std::str::from_utf8(&output2.stdout).unwrap(),
                std::str::from_utf8(&output3.stdout).unwrap()
            );

            let dialog = gtk::MessageDialog::new(
                Some(&window),
                gtk::DialogFlags::empty(),
                gtk::MessageType::Info,
                gtk::ButtonsType::Ok,
                &message
            );
            dialog.run();
            dialog.destroy();
        });
    }

    // GUIの実行
    gtk::main();
}
