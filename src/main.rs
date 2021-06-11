mod lib;

const FILE: &str = r#"@a @b

> a
> b

< a + b
"#;

fn main() {
    if let Err(e) = lib::run(FILE) {
        eprintln!("{}", e);
    }
}
