use std::{env, error::Error};
use log4rs;
use log::{error, info, warn};

pub(crate) fn main() {
      if env::var("RUST_LOG").is_err() {
              env::set_var("RUST_LOG", "info");
          }
     let log1=env::var("RUST_LOG").unwrap();
 
     println!("{}", log1);


     log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
     log::info!("INFO");
     log::warn!("helloq");
}