fn main() {
    if let Err(e) = tkc::run() {

        tkc::output::print_error(&e.to_string());
 
        std::process::exit(1);
    }
}