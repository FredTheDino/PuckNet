use std::path::PathBuf;

fn main() {
    let mut args = sylt::Args::parse_args_default_or_exit();
    args.file = Some(PathBuf::from(
        std::env::args()
        .nth(1)
        .as_ref()
        .map(|arg| arg.as_str())
        .unwrap_or("game.sy")
    ));
    println!("{:?}", args);

    if let Err(errs) = sylt::run_file(&args, sylt::lib_bindings()) {
        for e in errs.iter().take(5) {
            eprintln!("{}", e);
        }
        if errs.len() > 5 {
            eprintln!(">>> The other errors were ommited to save you scroll time")
        }
    }
}
