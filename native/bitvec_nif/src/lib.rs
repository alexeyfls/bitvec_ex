use std::sync::Mutex;

use bitvec::prelude::*;
use rustler::{Atom, Error as NifError, NifResult, Resource, ResourceArc};

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

#[rustler::nif]
fn new(capacity: usize) -> NifResult<ResourceArc<BitvecResource>> {
    let inner = bitvec![u8, Msb0; 0; capacity];
    let resource = ResourceArc::new(BitvecResource(Mutex::new(inner)));

    Ok(resource)
}

#[rustler::nif]
fn append(
    target: ResourceArc<BitvecResource>,
    source: ResourceArc<BitvecResource>,
) -> NifResult<bool> {
    let mut target_bits = target
        .0
        .lock()
        .map_err(|_| raise_error(atoms::lock_fail()))?;
    let mut source_bits = source
        .0
        .lock()
        .map_err(|_| raise_error(atoms::lock_fail()))?;

    target_bits.append(&mut source_bits);

    Ok(true)
}

#[rustler::nif]
fn capacity(resource: ResourceArc<BitvecResource>) -> NifResult<usize> {
    let guard = resource
        .0
        .lock()
        .map_err(|_| raise_error(atoms::lock_fail()))?;

    Ok(guard.capacity())
}

#[rustler::nif]
fn clear(resource: ResourceArc<BitvecResource>) -> NifResult<()> {
    let mut guard = resource
        .0
        .lock()
        .map_err(|_| raise_error(atoms::lock_fail()))?;

    Ok(guard.clear())
}

#[rustler::nif]
fn is_empty(resource: ResourceArc<BitvecResource>) -> NifResult<bool> {
    let guard = resource
        .0
        .lock()
        .map_err(|_| raise_error(atoms::lock_fail()))?;

    Ok(guard.is_empty())
}

#[rustler::nif]
fn len(resource: ResourceArc<BitvecResource>) -> NifResult<usize> {
    let guard = resource
        .0
        .lock()
        .map_err(|_| raise_error(atoms::lock_fail()))?;

    Ok(guard.len())
}

#[inline]
fn raise_error(atom: rustler::Atom) -> NifError {
    NifError::Term(Box::new(atom))
}

rustler::init!("Elixir.Bitvec.NifBridge");
