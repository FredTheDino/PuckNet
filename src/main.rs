use gumdrop::Options;

#[derive(Default, Debug, Options)]
struct Args {
    #[options(free)]
    args: Vec<String>,
    #[options(short = "v", no_long, count, help = "Increase verbosity, up to max 2")]
    verbosity: u32,
    #[options(help = "Print this help")]
    help: bool,
}

fn main() {
    let args = Args::parse_args_default_or_exit();

    let args = sylt::Args {
        verbosity: args.verbosity,
        args: args.args,

        ..sylt::Args::default()
    };

    if let Err(errs) = sylt::run_file(&args, sylt::lib_bindings()) {
        for e in errs.iter().take(5) {
            eprintln!("{}", e);
        }
        if errs.len() > 5 {
            eprintln!(">>> The other errors were ommited to save you scroll time")
        }
    }
}
