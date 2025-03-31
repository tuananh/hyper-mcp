use anyhow::Result;
use chrono::Local;
use log::LevelFilter;
use std::{io::Write, str::FromStr};

pub(crate) fn init_logger(path: Option<&str>, level: Option<&str>) -> Result<()> {
    let log_level = LevelFilter::from_str(level.unwrap_or("info"))?;

    // If the log path is not provided, use the stderr
    let log_file = match path {
        Some(p) => Box::new(
            std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(p)?,
        ) as Box<dyn Write + Send + Sync + 'static>,
        _ => Box::new(std::io::stderr()) as Box<dyn Write + Send + Sync + 'static>,
    };

    // TODO: apply module filter
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}/{}:{} {} [{}] - {}",
                record.module_path().unwrap_or("unknown"),
                basename(record.file().unwrap_or("unknown")),
                record.line().unwrap_or(0),
                Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                record.level(),
                record.args()
            )
        })
        .target(env_logger::Target::Pipe(log_file))
        .filter(None, log_level)
        .try_init()?;

    Ok(())
}

pub fn basename(path: &str) -> String {
    path.split('/').last().unwrap_or(path).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basename() {
        assert_eq!(basename("/path/to/file.txt"), "file.txt");
        assert_eq!(basename("file.txt"), "file.txt");
        assert_eq!(basename("file"), "file");
        assert_eq!(basename("/path/to/"), "");
        assert_eq!(basename(""), "");
    }

    #[test]
    fn test_init_logger() {
        // Test with a valid path
        let result = init_logger(Some("test.log"), Some("debug"));
        assert!(result.is_ok());

        // Test with an invalid path
        let result = init_logger(Some("/invalid/path/to/log.log"), Some("debug"));
        assert!(result.is_err());
    }
}
