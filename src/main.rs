#[allow(unused)]
struct TimeSample {
  inner: tokio::time::Instant,
}

#[allow(unused)]
impl TimeSample {
  fn new() -> Self {
    Self {
      inner: tokio::time::Instant::now(),
    }
  }
}

#[allow(unused)]
impl tracing_subscriber::fmt::time::FormatTime for TimeSample {
  fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
    let elapsed = self.inner.elapsed().as_secs();
    write!(w, "{:0>4}s", elapsed)
  }
}

#[cfg(debug_assertions)]
fn init_tracing() {
  use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt};

  let tracing_filter = tracing_subscriber::filter::Targets::new()
    .with_target("async", tracing::Level::TRACE)
    .with_target("main", tracing::Level::TRACE)
    .with_target("net", tracing::Level::TRACE)
    .with_target("mastary", tracing::Level::TRACE)
    .with_default(tracing::Level::ERROR);

  tracing_subscriber::registry()
    .with(
      tracing_subscriber::fmt::layer()
        .with_level(true)
        .with_target(true)
        .with_timer(TimeSample::new())
        .with_thread_names(true)
        .with_thread_ids(true),
    )
    .with(tracing_filter)
    .init();

  tracing::info!(target: "main", "trace level ...");
  tracing::warn!(target: "main" , "executing debug build: v{} ...", mastary::VERSION);
}

#[cfg(not(debug_assertions))]
fn init_tracing() {
  use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt};

  let tracing_filter = tracing_subscriber::filter::Targets::new()
    .with_target("async", tracing::Level::ERROR)
    .with_target("main", tracing::Level::ERROR)
    .with_target("net", tracing::Level::ERROR)
    .with_target("mastary", tracing::Level::ERROR)
    .with_default(tracing::Level::ERROR);

  tracing_subscriber::registry()
    .with(
      tracing_subscriber::fmt::layer()
        .with_level(true)
        .without_time()
        .with_target(true),
    )
    .with(tracing_filter)
    .init();
}



fn main() {
  init_tracing();

  mastary::init::init_ui();
}
