
// this file is a shared tracing module, which is used to trace the request and the response
use tracing::{error, info, warn, Level};
use tracing_subscriber::FmtSubscriber;

// exec main on start
pub fn init_trace() {
  let sub: FmtSubscriber = FmtSubscriber::builder()
    .with_max_level(Level::INFO)
    .finish();
  tracing::subscriber::set_global_default(sub).expect("setting default subscriber failed");
}


pub fn trace_logs(message: String){
  info!("{}", message);
}

pub fn trace_warn(message: String){
  warn!("{}", message);
}

pub fn trace_error(message: String){
  error!("{}", message);
}