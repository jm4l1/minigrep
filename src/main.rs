fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = minigrep::Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    // run(config).unwrap_or_else  also works but we do not need to 
    // the values returned by run() [which is () in this case].
    // we only need to check the error value. This is comparable with a call
    // to throw in C/C++
    if let Err(err) = minigrep::run(config) {
        println!("Encountered error: {err}");
    }
}
