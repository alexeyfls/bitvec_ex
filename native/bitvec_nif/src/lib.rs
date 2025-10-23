use std::sync::Mutex;

use bitvec::prelude::*;
use rustler::{nif, Atom, Resource, ResourceArc};

mod atoms {
    rustler::atoms! {
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

pub struct BitvecResource(Mutex<BitVec<usize, Msb0>>);

#[rustler::resource_impl]
impl Resource for BitvecResource {}

#[nif]
fn new(capacity: usize) -> Result<ResourceArc<BitvecResource>, Atom> {
    let inner = bitvec![usize, Msb0; 0; capacity];
    let resource = ResourceArc::new(BitvecResource(Mutex::new(inner)));

    Ok(resource)
}

#[nif]
fn len(resource: ResourceArc<BitvecResource>) -> Result<usize, Atom> {
    with_bitvec(&resource, |bits| Ok(bits.len()))
}

#[inline]
fn with_bitvec<F, T>(resource: &ResourceArc<BitvecResource>, f: F) -> Result<T, Atom>
where
    F: FnOnce(&BitVec<usize, Msb0>) -> Result<T, Atom>,
{
    let bits = resource.0.lock().map_err(|_| atoms::lock_fail())?;
    f(&*bits)
}

#[inline]
fn with_bitvec_mut<F, R>(resource: &ResourceArc<BitvecResource>, f: F) -> Result<R, Atom>
where
    F: FnOnce(&mut BitVec<usize, Msb0>) -> Result<R, Atom>,
{
    let mut guard = resource.0.lock().map_err(|_| atoms::lock_fail())?;
    f(&mut *guard)
}

rustler::init!("Elixir.Bitvec.NifBridge");
