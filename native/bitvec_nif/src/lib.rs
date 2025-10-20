use std::sync::Mutex;

use bitvec::prelude::*;
use rustler::{nif, Error as NifError, NifResult, Resource, ResourceArc};

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
impl Resource for BitvecResource {}

#[nif]
fn new(capacity: usize) -> NifResult<ResourceArc<BitvecResource>> {
    let inner = bitvec![u8, Msb0; 0; capacity];
    let resource = ResourceArc::new(BitvecResource(Mutex::new(inner)));

    Ok(resource)
}

#[nif]
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

#[nif]
fn capacity(resource: ResourceArc<BitvecResource>) -> NifResult<usize> {
    let bits = resource
        .0
        .lock()
        .map_err(|_| raise_error(atoms::lock_fail()))?;

    Ok(bits.capacity())
}

#[nif]
fn clear(resource: ResourceArc<BitvecResource>) -> NifResult {
    let mut bits = resource
        .0
        .lock()
        .map_err(|_| raise_error(atoms::lock_fail()))?;

    Ok(bits.clear())
}

#[nif]
fn insert(resource: ResourceArc<BitvecResource>, index: usize, value: bool) -> NifResult<()> {
    let mut bits = resource
        .0
        .lock()
        .map_err(|_| raise_error(atoms::lock_fail()))?;

    if bits.len() > index {
        return Err(raise_error(atoms::index_out_of_bounds()));
    }

    bits.insert(index, value);

    Ok(())
}

#[nif]
fn is_empty(resource: ResourceArc<BitvecResource>) -> NifResult<bool> {
    let bits = resource
        .0
        .lock()
        .map_err(|_| raise_error(atoms::lock_fail()))?;

    Ok(bits.is_empty())
}

#[nif]
fn len(resource: ResourceArc<BitvecResource>) -> NifResult<usize> {
    let bits = resource
        .0
        .lock()
        .map_err(|_| raise_error(atoms::lock_fail()))?;

    Ok(bits.len())
}

#[nif]
fn pop(resource: ResourceArc<BitvecResource>) -> NifResult<Option<bool>> {
    let mut bits = resource
        .0
        .lock()
        .map_err(|_| raise_error(atoms::lock_fail()))?;

    Ok(bits.pop())
}

#[nif]
fn pop(resource: ResourceArc<BitvecResource>, value: bool) -> NifResult<()> {
    let mut bits = resource
        .0
        .lock()
        .map_err(|_| raise_error(atoms::lock_fail()))?;

    Ok(bits.push(value))
}

#[nif]
fn remove(resource: ResourceArc<BitvecResource>, index: usize) -> NifResult<bool> {
    let mut bits = resource
        .0
        .lock()
        .map_err(|_| raise_error(atoms::lock_fail()))?;

    Ok(bits.remove(index))
}

#[nif]
fn repeat(bit: bool, len: usize) -> NifResult<ResourceArc<BitvecResource>> {
    let inner = BitVec::<u8, Msb0>::repeat(bit, len);
    let resource = ResourceArc::new(BitvecResource(Mutex::new(inner)));

    Ok(resource)
}

#[inline]
fn raise_error(atom: rustler::Atom) -> NifError {
    NifError::Term(Box::new(atom))
}

rustler::init!("Elixir.Bitvec.NifBridge");
