use std::env;
use std::io::{self, Error, ErrorKind};

fn main() -> io::Result<()> {
    let username = ["LOGNAME", "USER", "LNAME", "USERNAME"]
        .into_iter()
        .find_map(|name| env::var(name).ok().filter(|value| !value.is_empty()))
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "username is not available"))?;
    let length = username.chars().count();
    println!(
        r#"{{"version":1,"virtual_packages":{{"__username_length":{{"version":"{length}","build_string":"0"}}}}}}"#
    );
    Ok(())
}
