use std::env;
use clap::Shell;

include!("src/cli.rs");


fn main() {
    let outdir = env::var_os("CARGO_TARGET_DIR")
        .or_else(|| env::var_os("OUT_DIR"))
        .unwrap();

    let mut app = app();
    app.gen_completions("surface-dtx-userd", Shell::Bash, &outdir);
    app.gen_completions("surface-dtx-userd", Shell::Zsh,  &outdir);
    app.gen_completions("surface-dtx-userd", Shell::Fish, &outdir);
}
