use clap::{App, Arg, SubCommand};

pub fn opts<'a>() -> App<'a, 'a> {
  App::new("Rudiosti")
    .version("1.0")
    .about("Sound Processing")
    .arg(
      Arg::with_name("INPUT")
        .help("input file to process")
        .required(true)
        .index(1),
    )
    .subcommand(
      SubCommand::with_name("echo")
        .about("read and write out the INPUT file")
        .arg(
          Arg::with_name("INPUT")
            .help("input file to process")
            .required(true)
            .index(1),
        ),
    )
}
