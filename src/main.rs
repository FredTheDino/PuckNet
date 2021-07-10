use std::path::Path;

fn main() {
    let args = sylt::Args {
        file: Some(Path::new("game.sy").to_path_buf()),
        is_binary: false,
        compile_target: None,
        verbosity: 0,
        help: false,
        dump_tree: false,
        skip_typecheck: false,
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
