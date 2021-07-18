use gumdrop::Options;
use std::path::PathBuf;

#[derive(Default, Debug, Options)]
struct Args {
    #[options(short = "s", long = "server", help = "Start as a server")]
    is_server: bool,
    #[options(short = "c", long = "client", help = "Start as a client")]
    is_client: bool,
    #[options(short = "b", long = "browser", help = "Starts the server browser")]
    is_browser: bool,
    #[options(short = "p", long = "port", help = "The port to use when starting/connecting to a server")]
    port: u16,
    #[options(short = "v", no_long, count, help = "Increase verbosity, up to max 2")]
    verbosity: u32,
    #[options(help = "Print this help")]
    help: bool,
}

fn main() {
    let args = Args::parse_args_default_or_exit();

    let mut sylt_args = sylt::Args {
        file: if args.is_client {
            PathBuf::from("client.sy")
        } else if args.is_server {
            PathBuf::from("server.sy")
        } else if args.is_browser {
            PathBuf::from("browser.sy")
        } else {
            PathBuf::from("game.sy")
        },
        verbosity: args.verbosity,

        ..sylt::Args::default()
    };

    if args.is_browser {
        // Don't load lingon in the typechecker
        // sylt_args.skip_typecheck = true;
    }

    if let Err(errs) = sylt::run_file(&sylt_args, sylt::lib_bindings()) {
        for e in errs.iter().take(5) {
            eprintln!("{}", e);
        }
        if errs.len() > 5 {
            eprintln!(">>> The other errors were ommited to save you scroll time")
        }
    }
}
