enum OS {
    Windows(u32, String),
    Mac(u32, String),
    Linux(u32, String),
}

pub fn run()  {
    let linux = OS::Linux(1991, String::from("Linus"));
    print_os_info(linux);
    let windows = OS::Windows(1985, String::from("Microsoft"));
    print_os_info(windows);
    let mac = OS::Mac(2001, String::from("Apple"));
    print_os_info(mac);
}

// マッチングパターン
fn print_os_info(os: OS) {
    match os {
        OS::Windows(year, founder) => println!("Windows: {} {}", year, founder),
        OS::Mac(year, founder) => println!("Mac: {} {}", year, founder),
        OS::Linux(year, founder) => println!("Linux: {} {}", year, founder),
    }
}