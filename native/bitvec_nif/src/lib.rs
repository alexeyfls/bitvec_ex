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
fn new() -> Result<ResourceArc<BitvecResource>, Atom> {
    let inner = BitVec::<usize, Msb0>::new();
    let resource = ResourceArc::new(BitvecResource(Mutex::new(inner)));

    Ok(resource)
}

#[nif]
fn with_capacity(capacity: usize) -> Result<ResourceArc<BitvecResource>, Atom> {
    let inner = BitVec::<usize, Msb0>::with_capacity(capacity);
    let resource = ResourceArc::new(BitvecResource(Mutex::new(inner)));

    Ok(resource)
}

#[nif]
fn capacity(resource: ResourceArc<BitvecResource>) -> Result<usize, Atom> {
    with_bitvec(&resource, |bits| Ok(bits.capacity()))
}

#[nif]
fn reserve(resource: ResourceArc<BitvecResource>, additional: usize) -> Result<(), Atom> {
    with_bitvec_mut(&resource, |bits| Ok(bits.reserve(additional)))
}

#[nif]
fn reserve_exact(resource: ResourceArc<BitvecResource>, additional: usize) -> Result<(), Atom> {
    with_bitvec_mut(&resource, |bits| Ok(bits.reserve_exact(additional)))
}

#[nif]
fn shrink_to_fit(resource: ResourceArc<BitvecResource>) -> Result<(), Atom> {
    with_bitvec_mut(&resource, |bits| Ok(bits.shrink_to_fit()))
}

#[nif]
fn truncate(resource: ResourceArc<BitvecResource>, new_len: usize) -> Result<(), Atom> {
    with_bitvec_mut(&resource, |bits| Ok(bits.truncate(new_len)))
}

#[nif]
fn swap_remove(resource: ResourceArc<BitvecResource>, index: usize) -> Result<bool, Atom> {
    with_bitvec_mut(&resource, |bits| Ok(bits.swap_remove(index)))
}

#[nif]
fn insert(resource: ResourceArc<BitvecResource>, index: usize, value: bool) -> Result<(), Atom> {
    with_bitvec_mut(&resource, |bits| Ok(bits.insert(index, value)))
}

#[nif]
fn remove(resource: ResourceArc<BitvecResource>, index: usize) -> Result<bool, Atom> {
    with_bitvec_mut(&resource, |bits| Ok(bits.remove(index)))
}

#[nif]
fn push(resource: ResourceArc<BitvecResource>, value: bool) -> Result<(), Atom> {
    with_bitvec_mut(&resource, |bits| Ok(bits.push(value)))
}

#[nif]
fn pop(resource: ResourceArc<BitvecResource>) -> Result<Option<bool>, Atom> {
    with_bitvec_mut(&resource, |bits| Ok(bits.pop()))
}

#[nif]
fn append(
    target: ResourceArc<BitvecResource>,
    other: ResourceArc<BitvecResource>,
) -> Result<(), Atom> {
    with_bitvec_mut(&other, |other_bits| {
        with_bitvec_mut(&target, |target_bits| Ok(target_bits.append(other_bits)))
    })
}

#[nif]
fn clear(resource: ResourceArc<BitvecResource>) -> Result<(), Atom> {
    with_bitvec_mut(&resource, |bits| Ok(bits.clear()))
}

#[nif]
fn len(resource: ResourceArc<BitvecResource>) -> Result<usize, Atom> {
    with_bitvec(&resource, |bits| Ok(bits.len()))
}

#[nif]
fn is_empty(resource: ResourceArc<BitvecResource>) -> Result<bool, Atom> {
    with_bitvec(&resource, |bits| Ok(bits.is_empty()))
}

#[nif]
fn split_off(
    resource: ResourceArc<BitvecResource>,
    at: usize,
) -> Result<ResourceArc<BitvecResource>, Atom> {
    with_bitvec_mut(&resource, |bits| {
        let inner = bits.split_off(at);
        let resource = ResourceArc::new(BitvecResource(Mutex::new(inner)));

        Ok(resource)
    })
}

#[nif]
fn resize(resource: ResourceArc<BitvecResource>, new_len: usize, value: bool) -> Result<(), Atom> {
    with_bitvec_mut(&resource, |bits| Ok(bits.resize(new_len, value)))
}

#[nif]
fn repeat(bit: bool, len: usize) -> Result<ResourceArc<BitvecResource>, Atom> {
    let inner = BitVec::<usize, Msb0>::repeat(bit, len);
    let resource = ResourceArc::new(BitvecResource(Mutex::new(inner)));

    Ok(resource)
}

#[nif]
fn from_vec(vec: Vec<usize>) -> Result<ResourceArc<BitvecResource>, Atom> {
    let inner = BitVec::<usize, Msb0>::from_vec(vec);
    let resource = ResourceArc::new(BitvecResource(Mutex::new(inner)));

    Ok(resource)
}

#[nif]
fn into_vec(resource: ResourceArc<BitvecResource>) -> Result<Vec<usize>, Atom> {
    with_bitvec_mut(&resource, |bits: &mut BitVec<usize, Msb0>| {
        Ok(bits.clone().into_vec())
    })
}

#[nif]
fn set_uninitialized(resource: ResourceArc<BitvecResource>, value: bool) -> Result<(), Atom> {
    with_bitvec_mut(&resource, |bits| Ok(bits.set_uninitialized(value)))
}

#[nif]
fn force_align(resource: ResourceArc<BitvecResource>) -> Result<(), Atom> {
    with_bitvec_mut(&resource, |bits| Ok(bits.force_align()))
}

#[nif]
fn swap(resource: ResourceArc<BitvecResource>, a: usize, b: usize) -> Result<(), Atom> {
    with_bitvec_mut(&resource, |bits| Ok(bits.swap(a, b)))
}

#[nif]
fn reverse(resource: ResourceArc<BitvecResource>) -> Result<(), Atom> {
    with_bitvec_mut(&resource, |bits| Ok(bits.reverse()))
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
