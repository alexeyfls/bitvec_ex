use std::sync::Mutex;

use bitvec::prelude::*;
use rustler::{Atom, Resource, ResourceArc};

mod atoms {
    rustler::atoms! {
        ok,
        error,
        bad_reference,
        lock_fail,
        added,
        duplicate,
        removed,
        unsupported_type,
        not_found,
        index_out_of_bounds,
        max_bucket_size_exceeded,
    }
}

pub struct BitvecResource(Mutex<BitVec<u8, Msb0>>);

#[rustler::resource_impl]
impl Resource for BitvecResource {
    const IMPLEMENTS_DESTRUCTOR: bool = false;
}

type NifResult<T> = Result<(Atom, T), Atom>;

#[rustler::nif]
fn new(capacity: usize) -> NifResult<ResourceArc<BitvecResource>> {
    let inner = bitvec![u8, Msb0; 0; capacity];
    let resource = ResourceArc::new(BitvecResource(Mutex::new(inner)));

    Ok((atoms::ok(), resource))
}

#[rustler::nif]
fn len(resource: ResourceArc<BitvecResource>) -> NifResult<usize> {
    let guard = resource.0.lock().map_err(|_| atoms::lock_fail())?;

    Ok((atoms::ok(), guard.len()))
}

rustler::init!("Elixir.Bitvec.NifBridge");
