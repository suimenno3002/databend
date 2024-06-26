// Copyright [2021] [Jorge C Leitao]
// Copyright 2021 Datafuse Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::convert::TryInto;

use super::Packed;
use super::Unpackable;
use super::Unpacked;

/// Encodes (packs) a slice of [`Unpackable`] into bitpacked bytes `packed`, using `num_bits` per value.
///
/// This function assumes that the maximum value in `unpacked` fits in `num_bits` bits
/// and saturates higher values.
///
/// Only the first `ceil8(unpacked.len() * num_bits)` of `packed` are populated.
pub fn encode<T: Unpackable>(unpacked: &[T], num_bits: usize, packed: &mut [u8]) {
    let chunks = unpacked.chunks_exact(T::Unpacked::LENGTH);

    let remainder = chunks.remainder();

    let packed_size = (T::Unpacked::LENGTH * num_bits + 7) / 8;
    if !remainder.is_empty() {
        let packed_chunks = packed.chunks_mut(packed_size);
        let mut last_chunk = T::Unpacked::zero();
        for i in 0..remainder.len() {
            last_chunk[i] = remainder[i]
        }

        chunks
            .chain(std::iter::once(last_chunk.as_ref()))
            .zip(packed_chunks)
            .for_each(|(unpacked, packed)| {
                T::pack(&unpacked.try_into().unwrap(), num_bits, packed);
            });
    } else {
        let packed_chunks = packed.chunks_exact_mut(packed_size);
        chunks.zip(packed_chunks).for_each(|(unpacked, packed)| {
            T::pack(&unpacked.try_into().unwrap(), num_bits, packed);
        });
    }
}

/// Encodes (packs) a potentially incomplete pack of [`Unpackable`] into bitpacked
/// bytes `packed`, using `num_bits` per value.
///
/// This function assumes that the maximum value in `unpacked` fits in `num_bits` bits
/// and saturates higher values.
///
/// Only the first `ceil8(unpacked.len() * num_bits)` of `packed` are populated.
#[inline]
pub fn encode_pack<T: Unpackable>(unpacked: &[T], num_bits: usize, packed: &mut [u8]) {
    if unpacked.len() < T::Packed::LENGTH {
        let mut complete_unpacked = T::Unpacked::zero();
        complete_unpacked.as_mut()[..unpacked.len()].copy_from_slice(unpacked);
        T::pack(&complete_unpacked, num_bits, packed)
    } else {
        T::pack(&unpacked.try_into().unwrap(), num_bits, packed)
    }
}
