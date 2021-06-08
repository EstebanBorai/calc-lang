mod lib;

const FILE: &str = r#"@a @b

> a
> b

< a + b
"#;

fn main() {
    lib::run(FILE);
}
