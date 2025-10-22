use std::sync::Mutex;

use bitvec::prelude::*;
use rustler::{nif, Atom, Resource, ResourceArc};

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

        lsb0,
        msb0,
    }
}

enum InnerBitVec {
    Msb(BitVec<u8, Msb0>),
    Lsb(BitVec<u8, Lsb0>),
}

pub struct BitvecResource(Mutex<InnerBitVec>);

#[rustler::resource_impl]
impl Resource for BitvecResource {}

macro_rules! with_bitvec {
    ($resource:expr, $bv:ident => $body:expr) => {{
        let guard = $resource.0.lock().map_err(|_| atoms::lock_fail())?;
        match &*guard {
            InnerBitVec::Msb($bv) => $body,
            InnerBitVec::Lsb($bv) => $body,
        }
    }};
    (mut $resource:expr, $bv:ident => $body:expr) => {{
        let mut guard = $resource.0.lock().map_err(|_| atoms::lock_fail())?;
        match &mut *guard {
            InnerBitVec::Msb($bv) => $body,
            InnerBitVec::Lsb($bv) => $body,
        }
    }};
}

#[nif]
fn new(capacity: usize, ordering: Atom) -> Result<ResourceArc<BitvecResource>, Atom> {
    let inner = match ordering {
        a if a == atoms::msb0() => InnerBitVec::Msb(bitvec![u8, Msb0; 0; capacity]),
        a if a == atoms::lsb0() => InnerBitVec::Lsb(bitvec![u8, Lsb0; 0; capacity]),
        _ => return Err(atoms::unsupported_type()),
    };

    let resource = ResourceArc::new(BitvecResource(Mutex::new(inner)));

    Ok(resource)
}

#[nif]
fn len(resource: ResourceArc<BitvecResource>) -> Result<usize, Atom> {
    with_bitvec!(resource, bits => Ok(bits.len()))
}

rustler::init!("Elixir.Bitvec.NifBridge");
