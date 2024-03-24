
#[tokio::main]
async fn main() {
  println!("Starting bluest test");
  get_ble();
}

use futures::{executor::block_on, StreamExt};

fn get_ble() -> () {

  let adapter = block_on(bluest::Adapter::default());
  
  if let Some(adapter) = adapter {
    println!("Found default adapter");

    if let Ok(_) = block_on(adapter.wait_available()) {
      println!("Adapter available");

      if let Ok(mut scan) = block_on(adapter.scan(&[])) {
        
        while let Some(discovered_device) = block_on(scan.next()) {
          println!(
              "{:?}",
              discovered_device.device.name().as_deref().unwrap_or("unknown")
            );
          }
        }
    }

  } else {
    println!("No default adapter found.");
  }
  
}
