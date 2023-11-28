extern crate zmq;

use std::thread;
use std::time::Duration;

fn main() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    let mut msg = zmq::Message::new();
    loop {
        //Gets message
        responder.recv(&mut msg, 0).unwrap();
        println!("Message Received");//{}", msg.as_str().unwrap());

        //Delimits string to seperate the values
        let mut msgDelimited: Vec<String> = msg.as_str().unwrap().split(":").map(|s| s.to_string()).collect();
        let mut coords: Vec<f64> = Vec::new();

        //Converts string vector into vector of floats
        for n in msgDelimited.iter_mut() {
            let mut val = n.parse().unwrap();
            coords.push(val);
            println!("{}",val);
        }

        //Sends some files back
        let file1 = String::from("m1.geotiff");
        let file2 = String::from("m2.dt2");
        let file3 = String::from("m3.geojson");

        let response = file1 + ":" + &*file2 + ":" + &*file3;

        responder.send(&response, 0).unwrap();
    }
}

