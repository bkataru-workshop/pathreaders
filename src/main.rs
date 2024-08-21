use std::env;

const PATH_VAR_NAME: &str = "PATH";

fn main() {
    let path_string = match env::var(PATH_VAR_NAME) {
        Ok(path_string) => path_string,
        Err(error) => {
            panic!("Unable to parse variable with given PATH_VAR_NAME because of {error}");
        },
    };

    let paths = path_string.split(":").into_iter();

    for path in paths {
        println!("{path}");
    }

}
