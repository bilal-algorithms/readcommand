pub mod config {
    pub struct Config {
        pub file_path: String,
    }

    impl Config {
        pub fn new(args: &[String]) -> Self {
            let file_path = args[1].clone();

            Self {
                file_path,
            }
        }
    }
}


pub fn parse_config(args: &[String]) -> config::Config {
    config::Config { file_path: args[1].clone() }
}
