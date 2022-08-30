use criterion::{black_box, criterion_group, criterion_main, Criterion};
use minijinja::machinery::parse;
use minijinja::{context, Environment, State};

fn do_parse() {
    parse(
        black_box(include_str!("../inputs/all_elements.html")),
        "all_elements.html",
    )
    .unwrap();
}

fn do_parse_and_compile() {
    let mut env = Environment::new();
    env.add_template(
        "all_elements.html",
        include_str!("../inputs/all_elements.html"),
    )
    .unwrap();
}

fn do_render(env: &Environment) {
    let tmpl = env.get_template("all_elements.html").unwrap();
    tmpl.render(context! {
        DEBUG => false,
        site => context! {
            nav => vec![
                context!{url => "/", is_active => true, title => "Index"},
                context!{url => "/doc", is_active => false, title => "Docs"},
                context!{url => "/help", is_active => false, title => "Help"},
            ]
        }
    })
    .unwrap();
}

fn create_real_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.add_template("footer.html", include_str!("../inputs/footer.html"))
        .unwrap();
    env.add_template(
        "all_elements.html",
        include_str!("../inputs/all_elements.html"),
    )
    .unwrap();
    env.add_filter("asset_url", |_: &State, value: String| Ok(value));
    env.add_function("current_year", |_: &State| Ok(2022));
    env
}

fn create_macro_env() -> Environment<'static> {
    let mut env = Environment::new();
    let my_macro = r"{% macro greet(first, second, third=[1,2,3]) %}
    first is {{first}}
    second is {{second}}
    third is {{third}}
{% endmacro %}";
    env.add_macro("greet", my_macro);

    env.add_template("macro.html", include_str!("../inputs/macro.html"))
        .unwrap();

    env
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("render basic macro", |b| {
        let env = create_macro_env();
        let tmpl = env.get_template("macro.html").unwrap();
        b.iter(|| {
            tmpl.render(context! {DEBUG => false})
            .unwrap();
        });
    });

    // c.bench_function("parse all_elements", |b| b.iter(|| do_parse()));
    // c.bench_function("parse+compile all_elements", |b| {
    //     b.iter(|| do_parse_and_compile())
    // });
    // c.bench_function("render all_elements", |b| {
    //     let env = create_real_env();
    //     b.iter(|| do_render(&env));
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
