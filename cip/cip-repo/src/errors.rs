pub use crate::docs_toml::CipDiagnostic;

pub fn print_and_exit(diags: &[CipDiagnostic]) -> ! {
    for d in diags {
        eprintln!("{}", d.render());
    }
    std::process::exit(1);
}
