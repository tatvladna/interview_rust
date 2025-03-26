mod static_dynamic;
mod monomorphization;
mod errors;
mod multithreading;
mod rw_lock;
mod zst;
mod variance;
mod trait_object;
mod box_vec_arc;
mod cntx_static;
mod buffer;



use static_dynamic::static_dyn;
use monomorphization::monomorphization;
use errors::errors;
use multithreading::multithreading;
use rw_lock::rw_lock;
use zst::zst;
use variance::variance;
use trait_object::trait_object;
use box_vec_arc::box_vec_arc;
use cntx_static::cntx_static;
use buffer::buffer;


fn main () {
    static_dyn();
    monomorphization();
    errors();
    multithreading();
    rw_lock();
    zst();
    variance();
    trait_object();
    box_vec_arc();
    cntx_static();
    match buffer() {
        Ok(b) => println!("{:?}", b),
        Err(e) => eprintln!("Error: {}", e),
    }
}