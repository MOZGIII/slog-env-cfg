
use slog::Drain;
use std::str::FromStr;
pub enum LogFormat {
    Terminal,
    Json,
}

impl FromStr for LogFormat {
    type Err = &'static str;

    fn from_str(item: &str) -> Result<Self, Self::Err> {
        match item.to_lowercase().as_ref() {
            "terminal" | "term" => Ok(LogFormat::Terminal),
            "json" => Ok(LogFormat::Json),
            _ => Err("invalid log format"),
        }
    }
}

pub struct Config {
    pub format: LogFormat,
}

impl Config {
    pub fn build(&self) -> impl slog::Drain<Ok = (), Err = slog::Never> {
        let drain: Box<dyn slog::Drain<Ok = (), Err = slog::Never> + Send> = match self.format {
            LogFormat::Terminal => Box::new(self.build_terminal()),
            LogFormat::Json => Box::new(self.build_json()),
        };
        slog_async::Async::new(drain).build().fuse()
    }

    fn build_terminal(&self) -> impl slog::Drain<Ok = (), Err = slog::Never> {
        let decorator = slog_term::TermDecorator::new().stdout().build();
        slog_term::CompactFormat::new(decorator).build().fuse()
    }

    fn build_json(&self) -> impl slog::Drain<Ok = (), Err = slog::Never> {
        slog_json::Json::default(std::io::stdout()).fuse()
    }
}

#[test]
fn build_test_terminal() {
    let cfg = Config {
        format: LogFormat::Terminal,
    };
    cfg.build();
}


#[test]
fn build_test_json() {
    let cfg = Config {
        format: LogFormat::Json,
    };
    cfg.build();
}
