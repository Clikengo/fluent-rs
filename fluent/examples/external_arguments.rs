extern crate fluent;

use fluent::bundle::FluentBundle;
use fluent::types::FluentValue;
use std::collections::HashMap;

fn main() {
    let mut bundle = FluentBundle::new(&["en"]);
    bundle
        .add_messages(
            "
hello-world = Hello { $name }
ref = The previous message says { hello-world }
unread-emails =
    { $emailCount ->
        [one] You have { $emailCount } unread email
       *[other] You have { $emailCount } unread emails
    }
",
        )
        .unwrap();

    let mut args = HashMap::new();
    args.insert("name", FluentValue::from("John"));

    match bundle.format("hello-world", Some(&args)) {
        Some((value, _)) => println!("{}", value),
        _ => println!("None"),
    }

    match bundle.format("ref", Some(&args)) {
        Some((value, _)) => println!("{}", value),
        _ => println!("None"),
    }

    let mut args = HashMap::new();
    args.insert("emailCount", FluentValue::as_number("1.0").unwrap());

    match bundle.format("unread-emails", Some(&args)) {
        Some((value, _)) => println!("{}", value),
        None => println!("None"),
    }
}
