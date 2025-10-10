use ana::toml::parser::TomlParser;
use ana::toml::scanner::TomlScanner;

// Example usage
fn main() -> Result<(), std::io::Error> {
    let mut sc = TomlScanner::new("./test_data/test.toml");
    let _ = sc.scan()?;

    let mut parser = TomlParser::new();
    match parser.parse(sc.tokens) {
        Ok(_) => {}
        Err(error) => {
            println!("{}", error);
        }
    }

    println!("{}", parser.toml);
    Ok(())
}
