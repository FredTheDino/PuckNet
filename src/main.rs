use gumdrop::Options;
use std::path::PathBuf;

#[derive(Default, Debug, Options)]
struct Args {
    #[options(short = "s", long = "server", help = "Start as a server")]
    is_server: bool,
    #[options(short = "l", long = "local", help = "Start as a local instance")]
    is_local: bool,
    #[options(short = "p", long = "port", help = "The port to use when starting/connecting to a server")]
    port: u16,
    #[options(short = "v", no_long, count, help = "Increase verbosity, up to max 2")]
    verbosity: u32,
    #[options(help = "Print this help")]
    help: bool,
}

fn main() {
    let args = Args::parse_args_default_or_exit();

    let args = sylt::Args {
        file: if args.is_local {
            PathBuf::from("game.sy")
        } else if args.is_server {
            PathBuf::from("server.sy")
        } else {
            PathBuf::from("client.sy")
        },
        verbosity: args.verbosity,

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
