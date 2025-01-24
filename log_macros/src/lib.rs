#[macro_export]
macro_rules! loc {
    () => {
        format!("{}:{}", file!(), line!())
    };
}

#[macro_export]
macro_rules! log_error {
    ($err:expr) => {{
        let error = anyhow::anyhow!($err);
        log::error!("{:?} [{}:{}]",error, file!(), line!());
        error
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        let error = anyhow::anyhow!(format!($fmt, $($arg)*));
        log::error!("{:?} [{}:{}]", error, file!(), line!());
        error
    }};
}

#[macro_export]
macro_rules! log_warning {
    ($msg:expr) => {{
        log::warning!("{} [{}:{}]", $msg, file!(), line!());
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        log::warning!("{} [{}:{}]", format!($fmt, $($arg)*), file!(), line!(), );
    }};
}

#[macro_export]
macro_rules! log_info {
    ($msg:expr) => {{
        log::info!("{} [{}:{}]", $msg, file!(), line!());
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        log::info!("{} [{}:{}]", format!($fmt, $($arg)*), file!(), line!(), );
    }};
}

#[macro_export]
macro_rules! log_debug {
    ($msg:expr) => {{
        log::debug!("{} [{}:{}]", $msg, file!(), line!());
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        log::debug!("{} [{}:{}]", format!($fmt, $($arg)*), file!(), line!(), );
    }};
}

pub use log;
