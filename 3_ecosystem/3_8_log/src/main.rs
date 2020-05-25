#[macro_use]
extern crate log;
#[macro_use(slog_info, slog_o)]
extern crate slog;

use crate::slog::Drain;
use slog::Record;
use slog::{Duplicate, Level, LevelFilter};
use slog::{FnValue, PushFnValue};
use std::sync::Mutex;

fn main() {
    let app_log = Mutex::new(slog_json::Json::new(std::io::stdout()).build()).fuse();
    let std_err = Mutex::new(slog_json::Json::new(std::io::stderr()).build()).fuse();
    let global = slog::Logger::root(
        Duplicate::new(
            LevelFilter::new(app_log, Level::Info).filter(|x| x.level() > Level::Warning),
            LevelFilter::new(std_err, Level::Warning),
        )
        .fuse(),
        slog_o!(
            "msg" => PushFnValue(move |record : &Record, ser| {
                ser.emit(record.msg())
            }),
            "time" => PushFnValue(move |_ : &Record, ser| {
                    ser.emit(chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Nanos, true))
                }),
            "file" => "app.log",
            "lvl" => FnValue(move |rec : &Record| {
                    rec.level().to_string()
                }),
        ),
    );

    slog_stdlog::init().expect("err");
    let guard = slog_scope::set_global_logger(global);

    let local = slog::Logger::root(
        Mutex::new(
            slog_json::Json::new(
                std::fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open("access.log")
                    .expect("error during log file opening"),
            )
            .build(),
        )
        .fuse(),
        slog_o!(
                "msg" => PushFnValue(move |record : &Record, ser| {
                    ser.emit(record.msg())
                }),
                "time" => PushFnValue(move |_ : &Record, ser| {
                        ser.emit(chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Nanos, true))
                    }),
                "file" => "access.log",
                "lvl" => FnValue(move |rec : &Record| {
                        rec.level().to_string()
                    }),
        ),
    );
    slog_info!(local, "local log");

    info!("aaaaaaaa");
    warn!("bbbbbbb");
    info!("cccccccc");
}
