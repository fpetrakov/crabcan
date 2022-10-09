use cli::setup_log;

mod cli;

fn main() {
    let args = cli::parse_args();

    if args.debug {
        setup_log(log::LevelFilter::Debug)
    } else {
        setup_log(log::LevelFilter::Info)
    }

    log::info!("{:?}", args);
}
