rustc --crate-type=lib lib\gameSettings\modes.rs
rustc src/main.rs --extern modes=libmodes.rlib