use clap::{App, Arg};

pub fn build_args() -> App<'static, 'static> {
    App::new("proxy-mode-json")
    .about("Emits JSON to configure Microsoft Edge's ProxySetting GPO.")
    .version("1.0")
    .author("Robert Cato <rcato@ci.thibodaux.la.us>")
    .arg(
        Arg::with_name("pretty-print")
            .help("Emit formatted 'pretty' JSON.")
            .short("f")
            .long("pretty")
            .required(false)
    )
    .arg(
        Arg::with_name("mode")
            .help("Proxy Mode.")
            .short("m")
            .long("mode")
            .index(1)
            .required(true)
            .takes_value(true)
    )
    .arg(
        Arg::with_name("pac-url")
            .help("URL containing proxy .pac file.")
            .short("u")
            .long("pac-url")
            .required(false)
            .takes_value(true)
    )
    .arg(
        Arg::with_name("proxy-server")
            .help("URI of proxy server including port.")
            .short("s")
            .long("server")
            .required(false)
            .takes_value(true)
    )
    .arg(
        Arg::with_name("proxy-bypass-list")
            .help("List of IP addresses to bypass proxy.")
            .short("b")
            .long("bypass-list")
            .required(false)
            .takes_value(true)
    ) 
}