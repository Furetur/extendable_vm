use crate::jex::make_jex::make_jex_machine;

mod jex;
mod string_interner;
mod machine;

fn main() {
    let path = std::env::args().nth(1).expect("Filepath not given");
    let mut machine = make_jex_machine(&path)?;
    machine.run();
}
