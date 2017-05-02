extern crate neon_build;
extern crate gcc;

#[cfg(all(windows))]
fn main() {
    use std::process::Command;
    if cfg!(target_env = "msvc") {
        panic!("This project can't be used with MSVC.");
    }

    let mut where_cmd = Command::new("where");
    let where_output = where_cmd.arg(gcc::Config::new().get_compiler().path())
        .output()
        .unwrap_or_else(|error| {
            panic!("Failed to run where command: {}", error);
        });
    if !where_output.status.success() {
        panic!("\n{:?}\n{}\n{}\n",
               where_cmd,
               String::from_utf8_lossy(&where_output.stdout),
               String::from_utf8_lossy(&where_output.stderr));
    }
    let compiler_path_as_string = String::from_utf8_lossy(&where_output.stdout);
    println!("Found gcc in {:?}.", compiler_path_as_string.to_owned());

    neon_build::setup();
}

#[cfg(all(not(windows)))]
fn main() {
    neon_build::setup();
}
