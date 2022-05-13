use std::process::Command;
use std::thread;

fn init_cmd() -> Command {
    let mut cmd = Command::new("yt-dlp");
    cmd.arg("-f");
    cmd.arg("137+251");
    cmd
}

fn main() {
    let urls: Vec<String> = vec![
        "".to_owned(),
        "".to_owned(),
        "".to_owned(),
        "".to_owned(),
        "".to_owned(),
    ];

    let mut handles = vec![];

    for url in urls {
        let handle = thread::spawn(move || {
            let mut cmd = init_cmd();
            cmd.arg(url.clone());
            match cmd.output() {
                Ok(output) => {
                    println!("Downloading {}", url);
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
