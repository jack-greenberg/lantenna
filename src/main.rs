mod lantenna;

use clap::{App, Arg};
use lantenna::{init, send};
use std::io::Read;

fn main() -> Result<(), lantenna::LantennaError> {
    let args = App::new("lantenna")
        .arg(Arg::new("INPUT").about("File to send"))
        .arg(
            Arg::new("host")
                .about("The host to send the data from")
                .required(true)
                .short('h')
                .takes_value(true), // .validator(ip_validator)
        )
        .arg(
            Arg::new("target")
                .about("The target to send data to")
                .required(true)
                .short('t')
                .takes_value(true), // .validator(ip_validator)
        )
        .get_matches();


    if let Some(host) = args.value_of("host") {
        let socket = init(&host).unwrap();

        if let Some(target) = args.value_of("target") {
            match args.value_of("INPUT") {
                Some(file) => send(&socket, &target, std::fs::read(file).unwrap()),
                None => {
                    let mut v = Vec::new();
                    std::io::stdin().read_to_end(&mut v).unwrap();
                    send(&socket, &target, v)
                }
            };
        } else {
            println!("nah");
        }
    } else {
        println!("Nope");
    }

    Ok(())
}
