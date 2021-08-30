use clap::{App, Arg, ArgMatches};

pub fn run<'a>() -> ArgMatches<'a> {
	App::new("QinpelStp")
    .version("0.2.1")
    .author("Éverton M. Vieira <everton.muvi@gmail.com>")
    .about("QinpelStp - Qinpel Setup and Step")
    .arg(
      Arg::with_name("wait")
        .short("w")
        .long("wait")
        .value_name("NUMBER")
        .default_value("0")
        .takes_value(true)
        .required(false)
        .help("How long should I wait before to execute?"))
    .arg(
      Arg::with_name("install")
        .short("i")
        .long("install")
        .value_name("TYPE/NAME")
        .takes_value(true)
        .required(false)
        .help("What app or cmd should I install?"))
    .arg(
      Arg::with_name("run")
        .short("r")
        .long("run")
        .value_name("CMD")
        .takes_value(true)
        .required(false)
        .help("What command should I run?"))
    .get_matches()
}
