#[macro_export]
macro_rules! loc {
    () => {
        format!("{}:{}", file!(), line!())
    };
}

#[macro_export]
macro_rules! format_error {
    ($err:expr) => {{
        anyhow::anyhow!("{} [{}:{}]",$err, file!(), line!())
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        anyhow::anyhow!("{} [{}:{}]", format!($fmt, $($arg)*), file!(), line!(), )
    }};
}

#[macro_export]
macro_rules! log_error {
    ($err:expr) => {{
        let error = anyhow::anyhow!($err);
        log::error!("{}", error);
        error
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        let error = anyhow::anyhow!(format!($fmt, $($arg)*));
        log::error!("{}", error);
        error
    }};
}

#[macro_export]
macro_rules! log_warn {
    ($msg:expr) => {{
        log::warn!("{} [{}:{}]", $msg, file!(), line!());
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        log::warn!("{} [{}:{}]", format!($fmt, $($arg)*), file!(), line!(), );
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
