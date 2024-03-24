
#[tokio::main]
async fn main() {
  println!("Starting bluest test");
  get_ble();
}

use futures::{executor::block_on, StreamExt};

// https://bitbucket.org/bluetooth-SIG/public/src/main/assigned_numbers/

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
          println!("Connectable: {}", device.adv_data.is_connectable);
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
