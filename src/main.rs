use anyhow::{anyhow as ah, Result};

mod challenge;

fn main() -> Result<()> {
    color_backtrace::install();
    let args = get_args();
    setup_logger(args.occurrences_of("verbosity"));
    log::trace!("Args: {:?}", args);

    challenge::run(&args).map_err(|e| {
        log::error!("{}", e);
        ah!("unrecoverable failure")
    })
}
fn setup_logger(level: u64) {
    let mut builder = pretty_env_logger::formatted_timed_builder();

    let noisy_modules = &[
        "hyper",
        "mio",
        "tokio_core",
        "tokio_reactor",
        "tokio_threadpool",
        "fuse::request",
        "rusoto_core",
        "hab_core::dependencies::graph::exec_walker",
        "want",
    ];

    let log_level = match level {
        //0 => log::Level::Error,
        0 => log::LevelFilter::Warn,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    if level > 1 && level < 4 {
        for module in noisy_modules {
            builder.filter_module(module, log::LevelFilter::Info);
        }
    }

    builder.filter_level(log_level);
    builder.format_timestamp_millis();
    builder.init();
}
fn get_args() -> clap::ArgMatches<'static> {
    clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .setting(clap::AppSettings::DeriveDisplayOrder)
        .arg(
            clap::Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .global(true)
                .help("Sets the level of verbosity"),
        )
        .arg(clap::Arg::with_name("day").required(true))
        .arg(clap::Arg::with_name("part").required(true))
        .arg(clap::Arg::with_name("input").required(true))
        .get_matches()
}
