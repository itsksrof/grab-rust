use std::{env, fs};

fn main() {
    // Get the command arguments and store them in a Vec<String>
    let args: Vec<String> = env::args().collect();

    // Use the first command argument to determine which function should be executed
    match args[1].as_str() {
        "--help" => return println!("{}", get_help()),
        "-h" => return println!("{}", get_help()),
        _ => for arg in args.iter().skip(1) {
            match arg.as_str() {
                _ => print!("{}", get_file_content(arg))
            }
        }
    }
}

// get_help returns an example of how this program might be used
fn get_help() -> String {
    r#"Usage: grab [OPTION] ... [FILE] ...
Grab FILE(s) content and show it through standard output

--help, -h              display this help and exit

Examples:

grab foo.txt            Output 'foo.txt' contents
grab foo.txt bar.txt    Output 'foo.txt' contents, then, output 'bar.txt' contents"#.to_string()
}

// get_file_content returns the contents of the given file or an error string
fn get_file_content(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(file) => file,
        Err(error) => error.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_content() {
        // Create 'foo.txt'
        fs::write("foo.txt", "foo").unwrap();

        // Assert that 'foo.txt' contains 'foo'
        assert_eq!(get_file_content("foo.txt"), "foo");

        // Remove 'foo.txt'
        fs::remove_file("foo.txt").unwrap();
    }

    #[test]
    fn test_get_file_content_not_found() {
        // Assert that an error is returned on not found file
        assert_eq!(get_file_content("bar.txt"), "No such file or directory (os error 2)");
    }
}