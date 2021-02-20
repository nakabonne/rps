use clap::{crate_version, App, AppSettings, Arg, SubCommand};

use crate::cmd;

pub fn build_app() -> App<'static, 'static> {
    let clap_color_setting = if std::env::var_os("NO_COLOR").is_none() {
        AppSettings::ColoredHelp
    } else {
        AppSettings::ColorNever
    };

    let mut app = App::new("rps")
        .version(crate_version!())
        .usage("rps <cmd> <pid|addr>")
        .setting(clap_color_setting)
        .setting(AppSettings::DeriveDisplayOrder)
        .after_help(
            "Note: `rps -h` prints a short and concise overview while `rps --help` gives all \
                 details.",
        );

    // memstats command
    app = app.subcommand(SubCommand::with_name(cmd::CMD_MEM_STATS).about("Show memory stats"));

    app
}
