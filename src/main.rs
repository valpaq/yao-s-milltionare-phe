use concrete::prelude::*;
use concrete::{generate_keys, set_server_key, ConfigBuilder, FheUint16};
use rand::Rng;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();
    let config = ConfigBuilder::all_disabled()
        .enable_default_uint16()
        .build();
    let (keys, server_keys) = generate_keys(config);
    set_server_key(server_keys);
    let mut rng = rand::thread_rng();

    let elapsed_time = now.elapsed();
    println!("Building config took {} seconds.", elapsed_time.as_secs());
    let clear_b: u16 = rng.gen::<u16>() >> 1;
    let e_b = FheUint16::try_encrypt(clear_b, &keys)?;

    let clear_a: u16 = rng.gen::<u16>() >> 1;
    let random_r: u16 = rng.gen::<u16>();
    let e_a = FheUint16::try_encrypt(clear_a, &keys)?;
    let e_r = FheUint16::try_encrypt(random_r, &keys)?;
    let e_v = (e_a - e_b) ^ e_r;

    let v: u16 = e_v.decrypt(&keys);

    println!("A: {clear_a}");
    println!("B: {clear_b}",);
    let msb_v = (v >> 15) & 1;
    let msb_r = (random_r >> 15) & 1;
    let res = msb_v ^ msb_r;
    println!("Result: A {} B", if res == 0 { ">=" } else { "<" });

    let elapsed_time = now.elapsed();
    println!("Running main took {} seconds.", elapsed_time.as_secs());

    Ok(())
}
