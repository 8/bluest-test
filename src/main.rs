
#[tokio::main]
async fn main() {
  println!("Starting bluest test");
  get_ble();
}

use futures::{executor::block_on, StreamExt};

// https://bitbucket.org/bluetooth-SIG/public/src/main/assigned_numbers/

// Company: 0xFF0C
// Device: HR-70EC8EA6
// Rssi: Some(-66)
// Is Connectable: true
// Services: [0000180d-0000-1000-8000-00805f9b34fb]
// service uuid: 00001800-0000-1000-8000-00805f9b34fb
//         characteristics uuid: 00002a00-0000-1000-8000-00805f9b34fb
//         characteristics uuid: 00002a01-0000-1000-8000-00805f9b34fb
//         characteristics uuid: 00002a04-0000-1000-8000-00805f9b34fb
// service uuid: 00001801-0000-1000-8000-00805f9b34fb
// service uuid: 0000180d-0000-1000-8000-00805f9b34fb
//         characteristics uuid: 00002a37-0000-1000-8000-00805f9b34fb
//         characteristics uuid: 00002a38-0000-1000-8000-00805f9b34fb
// service uuid: 0000180f-0000-1000-8000-00805f9b34fb
//         characteristics uuid: 00002a19-0000-1000-8000-00805f9b34fb
// service uuid: 0000180a-0000-1000-8000-00805f9b34fb
//         characteristics uuid: 00002a29-0000-1000-8000-00805f9b34fb
//         characteristics uuid: 00002a24-0000-1000-8000-00805f9b34fb
//         characteristics uuid: 00002a27-0000-1000-8000-00805f9b34fb
//         characteristics uuid: 00002a26-0000-1000-8000-00805f9b34fb
//         characteristics uuid: 00002a28-0000-1000-8000-00805f9b34fb
// service uuid: 8fc3fd00-f21d-11e3-976c-0002a5d5c51b
//         characteristics uuid: 8fc3fd09-f21d-11e3-976c-0002a5d5c51b
//         characteristics uuid: 8fc3fd0a-f21d-11e3-976c-0002a5d5c51b
//         characteristics uuid: 8fc3fd15-f21d-11e3-976c-0002a5d5c51b
//         characteristics uuid: 8fc3fd16-f21d-11e3-976c-0002a5d5c51b

fn get_ble() -> () {

  let adapter = block_on(bluest::Adapter::default());
  
  if let Some(adapter) = adapter {
    println!("Found default adapter");

    if let Ok(_) = block_on(adapter.wait_available()) {
      println!("Adapter available");
      println!("Querying devices...");
      println!("");

      if let Ok(mut scan) = block_on(adapter.scan(&[])) {
        
        while let Some(device) = block_on(scan.next()) {
          println!("Company: {}", device.adv_data.manufacturer_data.map(|m|format!("0x{:02X}", m.company_id)).as_deref().unwrap_or("unknown"));
          println!("Device: {}", device.device.name().as_deref().unwrap_or("unknown"));
          println!("Rssi: {:?}", device.rssi);
          println!("Is Connectable: {}", device.adv_data.is_connectable);
          println!("Services: {:?}", device.adv_data.services);

          if let Ok(services) = block_on(device.device.discover_services()) {
            services.iter().for_each(|s| {
              let uuid = s.uuid();
              println!("service uuid: {}", uuid);
              if let Ok(characteristics) = block_on(s.characteristics()) {
                characteristics.iter().for_each(|c| {
                  println!("\tcharacteristics uuid: {}", c.uuid());
                });
              }

            });
          }

          println!("")
        }
      }
    }

  } else {
    println!("No default adapter found.");
  }
  
}
