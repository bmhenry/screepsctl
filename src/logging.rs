use stdweb::js;

pub use log::{LevelFilter::*, Log};

static LOGGER: JsLog = JsLog;
static mut LOGGER_QUEUE: String = String::new();

struct JsLog;
struct JsNotify;


impl log::Log for JsLog {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        metadata.level() <= log::max_level()
    }

    /// Queue up a log message, to be sent whenever flushed
    /// Should be flushed at the end of each tick
    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            let message = format!("[{}] ({}) {}\n", 
                record.metadata().level(), 
                record.metadata().target(), 
                record.args());
            unsafe {
                LOGGER_QUEUE.push_str(message.as_str());
            }
        }
    }

    /// This should be called at the end of a tick
    fn flush(&self) {
        unsafe {
            if !LOGGER_QUEUE.is_empty() {
                js! {
                    console.log(@{&LOGGER_QUEUE});
                }
                LOGGER_QUEUE.clear();
            }
        }
    }
}

impl log::Log for JsNotify {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            let message = format!("[{}] ({}) {}", 
                record.metadata().level(), 
                record.metadata().target(), 
                record.args());
            
            js! {
                console.log(@{message});
            }
        }
    }

    fn flush(&self) {}
}

pub fn setup_logging(verbosity: log::LevelFilter) -> Result<(), log::SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|_| log::set_max_level(verbosity))
}

pub fn store_log_level() {
    match log::max_level() {
        Off => screeps::memory::root().set("log_level", "off"),
        Trace => screeps::memory::root().set("log_level", "trace"),
        Debug => screeps::memory::root().set("log_level", "debug"),
        Info => screeps::memory::root().set("log_level", "info"),
        Warn => screeps::memory::root().set("log_level", "warn"),
        Error => screeps::memory::root().set("log_level", "error"),
    }
}

pub fn watch_log_level() {
    let mut level = Info;
    if let Ok(Some(val)) = screeps::memory::root().string("log_level") {
        match val.as_str() {
            "off" => level = Off,
            "trace" => level = Trace,
            "debug" => level = Debug,
            "info" => level = Info,
            "warn" => level = Warn,
            "error" => level = Error,
            _ => level = Info
        }
    }
    log::set_max_level(level);
}

pub fn print_logs() {
    LOGGER.flush();
}