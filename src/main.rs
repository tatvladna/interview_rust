mod static_dynamic;
mod monomorphization;
mod errors;
mod multithreading;


use static_dynamic::static_dyn;
use monomorphization::monomorphization;
use errors::errors;
use multithreading::multithreading;

fn main () {
    static_dyn();
    monomorphization();
    errors();
    multithreading();
}