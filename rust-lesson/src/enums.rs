enum OS {
    Windows(u32, String),
    Mac(u32, String),
    Linux(u32, String),
}

pub fn run() {
    let linux = OS::Linux(5, String::from("Ubuntu"));
    print_os_info(linux);
    let mac = OS::Mac(10, String::from("Monterey"));
    print_os_info(mac);
    let windows = OS::Windows(11, String::from("Windows 11"));
    print_os_info(windows);
}

fn print_os_info(os: OS) {
    match os {
        OS::Windows(version, name) => {
            println!("Windows version: {}, name: {}", version, name);
        }
        OS::Mac(version, name) => {
            println!("Mac version: {}, name: {}", version, name);
        }
        OS::Linux(version, name) => {
            println!("Linux version: {}, name: {}", version, name);
        }
    }
}
