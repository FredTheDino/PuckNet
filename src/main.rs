use std::path::Path;

fn main() {
    let mut args = sylt::Args::parse_args_default_or_exit();
    args.file = Some(Path::new("game.sy").to_path_buf());

    if let Err(errs) = sylt::run_file(&args, sylt::lib_bindings()) {
        for e in errs.iter().take(5) {
            eprintln!("{}", e);
        }
        if errs.len() > 5 {
            eprintln!(">>> The other errors were ommited to save you scroll time")
        }
    }
}
