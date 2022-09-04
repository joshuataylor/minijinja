use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

use minijinja::value::{FunctionArgs, Object, Value};
use minijinja::{Environment, Error, State};

fn main() {
    let my_macro = r"{% macro greet(first, second, third=[1,2,3]) %}
    first is {{first}}
    second is {{second}}
    third is {{third}}
{% endmacro %}";

    let mut env = Environment::new();
    env.add_macro("", my_macro);
    env.add_template(
        "demo.html",
        // "x"
        r#"{{greet('one', 'two')}}"#,
    )
    .unwrap();

    let tmpl = env.get_template("demo.html").unwrap();
    dbg!(&tmpl);
    dbg!(tmpl.render(&()));
    // println!("{}", tmpl.render(&()).unwrap());
}
