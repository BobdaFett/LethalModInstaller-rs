
#[macro_export]
macro_rules! flush {
  () => {
    io::stdout().flush().unwrap();
  };
}

pub use flush as flush;
