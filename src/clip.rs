use clap::{App, Arg, ArgMatches};

pub fn parse<'a>() -> ArgMatches<'a> {
	App::new("QinpelStp")
		.version(clap::crate_version!())
		.author("Éverton M. Vieira <everton.muvi@gmail.com>")
		.about("QinpelStp - Qinpel Setup and Step")
		.arg(
			Arg::with_name("os")
				.short("o")
				.long("os")
				.value_name("OS")
				.takes_value(true)
				.required(false)
				.help("For what operation system should I setup?"),
		)
		.arg(
			Arg::with_name("arch")
				.short("a")
				.long("arch")
				.value_name("ARCH")
				.takes_value(true)
				.required(false)
				.help("For what architecture should I setup?"),
		)
		.arg(
			Arg::with_name("wait")
				.short("w")
				.long("wait")
				.value_name("NUMBER")
				.default_value("0")
				.takes_value(true)
				.required(false)
				.help("How long should I wait before to execute?"),
		)
		.arg(
			Arg::with_name("index")
				.short("n")
				.long("index")
				.value_name("PATH")
				.takes_value(true)
				.required(false)
				.help("What path should I index?"),
		)
		.arg(
			Arg::with_name("install")
				.short("i")
				.long("install")
				.value_name("TYPE/NAME")
				.takes_value(true)
				.required(false)
				.help("What app or cmd should I install?"),
		)
		.arg(
			Arg::with_name("run")
				.short("r")
				.long("run")
				.value_name("CMD")
				.takes_value(true)
				.required(false)
				.help("What command should I run?"),
		)
		.arg(
			Arg::with_name("install-run")
				.short("x")
				.long("install-run")
				.value_name("CMD")
				.takes_value(true)
				.required(false)
				.help("What command should I install and run?"),
		)
		.get_matches()
}
