
#[tokio::main]
async fn main() {
  // println!("Hello, world!");
  println!("bluest test");
  get_ble();
}

fn get_ble() -> () {

  let adapter = futures::executor::block_on(bluest::Adapter::default());
  
  if let Some(adapter) = adapter {
    println!("Found default adapter");
  } else {
    println!("No default adapter found.");
  }

  
}
