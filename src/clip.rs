use clap::{Arg, ArgMatches, Command};

pub fn parse() -> ArgMatches {
    Command::new("QinpelStp")
		.version(clap::crate_version!())
		.about("QinpelStp ( Qinpel Setup ) is a command program that transfers, installs and executes selected bundles of user interfaces and command programs for the Qinpel, the Quick Interface for Pointel platform.")
		.author("Éverton M. Vieira <everton.muvi@gmail.com>")
		.arg(
			Arg::new("os")
				.short('o')
				.long("os")
				.value_name("OS")
				.takes_value(true)
				.required(false)
				.help("For what operation system should I setup?"),
		)
		.arg(
			Arg::new("arch")
				.short('a')
				.long("arch")
				.value_name("ARCH")
				.takes_value(true)
				.required(false)
				.help("For what architecture should I setup?"),
		)
		.arg(
			Arg::new("wait")
				.short('w')
				.long("wait")
				.value_name("NUMBER")
				.default_value("0")
				.takes_value(true)
				.required(false)
				.help("How long should I wait before to execute?"),
		)
		.arg(
			Arg::new("index")
				.short('n')
				.long("index")
				.value_name("PATH")
				.takes_value(true)
				.required(false)
				.help("What path should I index?"),
		)
		.arg(
			Arg::new("install")
				.short('i')
				.long("install")
				.value_name("TYPE/NAME")
				.takes_value(true)
				.required(false)
				.help("What app or cmd should I install?"),
		)
		.arg(
			Arg::new("run")
				.short('r')
				.long("run")
				.value_name("CMD")
				.takes_value(true)
				.required(false)
				.help("What command should I run?"),
		)
		.arg(
			Arg::new("install-run")
				.short('x')
				.long("install-run")
				.value_name("CMD")
				.takes_value(true)
				.required(false)
				.help("What command should I install and run?"),
		)
		.get_matches()
}
