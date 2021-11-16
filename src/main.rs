mod lantenna;

// use std::collections::HashMap;
use lantenna::{send, init};
// use config::Config;

fn main() {
    let socket = init("127.0.0.1:34254").unwrap();
    send(&socket, "127.0.0.1:34253", &[4]);
    // let mut settings = Config::default();
    // settings
    //     .merge(config::File::with_name("lantenna")).unwrap();
        // .clone()
        // .try_into::<HashMap<String, String>>()
        // .unwrap();
// 
//     println!("byte_time: {}", settings.get("byte_time").unwrap());
}
