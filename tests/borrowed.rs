extern crate clap;
extern crate regex;

use clap::{App, Arg};

include!("../clap-test.rs");

#[test]
fn borrowed_args() {
    let arg = Arg::new("some")
        .short("s")
        .long("some")
        .help("other help");
    let arg2 = Arg::new("some2")
        .short("S")
        .long("some-thing")
        .help("other help");
    let result = App::new("sub_command_negate")
        .arg(Arg::new("test").index(1))
        .arg(&arg)
        .arg(&arg2)
        .subcommand(App::new("sub1").arg(&arg))
        .get_matches_from_safe(vec!["prog"]);
    assert!(result.is_ok());
}
