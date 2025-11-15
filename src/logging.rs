use once_cell::sync::OnceCell;
use tracing_subscriber::EnvFilter;

static LOGGING: OnceCell<()> = OnceCell::new();

#[ctor::ctor]
fn _install_global_tracing() {
    LOGGING.get_or_init(|| {
        tracing_subscriber::fmt()
            .with_env_filter(
                EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
            )
            .with_test_writer()
            .with_target(true)
            .with_line_number(true)
            .with_ansi(false)
            .init();
    });
}
