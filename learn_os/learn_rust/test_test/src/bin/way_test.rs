fn foo() {
    loop {
        println!("aaa");
    }
}

fn join(s: &String) -> String {
    "HHH".to_string() + s
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if (s1.len() > s2.len()) {
        s1
    } else {
        s2
    }
}

struct Foo<'a> {
    part: &'a str,
}

fn main() {
    // let ff = foo();

    let x = "Hello".to_string();
    join(&x);


    let res = longest(&x, &x);
    println!("{}", x);
    println!("{}", res);
}
