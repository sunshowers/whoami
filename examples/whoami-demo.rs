fn main() {
    println!("WhoAmI {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!(
        "User's Language        whoami::langs():               {}",
        whoami::langs()
            .map(|l| l
                .map(|l| l.to_string())
                .unwrap_or_else(|_| "??".to_string()))
            .collect::<Vec<String>>()
            .join(", "),
    );
    println!(
        "User's Name            whoami::realname():            {}",
        whoami::realname(),
    );
    println!(
        "User's Username        whoami::username():            {}",
        whoami::username(),
    );
    println!(
        "Device's Pretty Name   whoami::devicename():          {}",
        whoami::devicename(),
    );
    println!(
        "Device's Hostname      whoami::fallible::hostname():  {}",
        whoami::fallible::hostname()
            .unwrap_or_else(|_| "localhost".to_string()),
    );
    println!(
        "Device's Platform      whoami::platform():            {}",
        whoami::platform(),
    );
    println!(
        "Device's OS Distro     whoami::distro():              {}",
        whoami::distro(),
    );
    println!(
        "Device's Desktop Env.  whoami::desktop_env():         {}",
        whoami::desktop_env(),
    );
    println!(
        "Device's CPU Arch      whoami::arch():                {}",
        whoami::arch(),
    );
}
