use readcommand::parse_config;
use readcommand::config::Config;

#[test]
fn test_parse_config() {
    let config = parse_config(&[String::from("file.txt")]);
    assert_eq!(config, Config { file_path: String::from("file.txt") });
}
