use std::io;
use std::io::ErrorKind;
use std::io::Read;

#[cfg(not(feature = "wsimd4k"))]
pub fn count_lines4k(s: &[u8; 4096], sep: u8) -> usize {
    s.iter().filter(|u| sep.eq(u)).count()
}

#[cfg(feature = "wsimd4k")]
use core::arch::wasm32;

#[cfg(feature = "wsimd4k")]
pub const V0: wasm32::v128 = wasm32::u64x2(0, 0);

#[cfg(feature = "wsimd4k")]
pub fn count_lines4k(s: &[u8; 4096], sep: u8) -> usize {
    let mut tot: usize = 0;
    let ptr: *const u8 = s.as_ptr();
    let v: *const wasm32::v128 = ptr as *const wasm32::v128;

    let vimax: usize = 4096 >> 4; // 256;

    #[allow(unsafe_code)]
    let vs: &[wasm32::v128] = unsafe { std::slice::from_raw_parts(v, vimax) };

    let hmax: usize = vimax >> 1; // 128

    let vsep: wasm32::v128 = wasm32::u8x16_splat(sep);

    let mut vtot0: wasm32::v128 = V0;
    for i in 0..hmax {
        let v: wasm32::v128 = vs[i];
        let b: wasm32::v128 = wasm32::u8x16_eq(v, vsep);
        let pcnt: wasm32::v128 = wasm32::u8x16_popcnt(b);
        let mcnt: wasm32::v128 = wasm32::u8x16_shr(pcnt, 3);
        vtot0 = wasm32::u8x16_add(mcnt, vtot0);
    }

    let mut vtot1: wasm32::v128 = V0;
    for i in hmax..vimax {
        let v: wasm32::v128 = vs[i];
        let b: wasm32::v128 = wasm32::u8x16_eq(v, vsep);
        let pcnt: wasm32::v128 = wasm32::u8x16_popcnt(b);
        let mcnt: wasm32::v128 = wasm32::u8x16_shr(pcnt, 3);
        vtot1 = wasm32::u8x16_add(mcnt, vtot1);
    }

    tot += wasm32::u8x16_extract_lane::<0>(vtot0) as usize;
    tot += wasm32::u8x16_extract_lane::<1>(vtot0) as usize;
    tot += wasm32::u8x16_extract_lane::<2>(vtot0) as usize;
    tot += wasm32::u8x16_extract_lane::<3>(vtot0) as usize;
    tot += wasm32::u8x16_extract_lane::<4>(vtot0) as usize;
    tot += wasm32::u8x16_extract_lane::<5>(vtot0) as usize;
    tot += wasm32::u8x16_extract_lane::<6>(vtot0) as usize;
    tot += wasm32::u8x16_extract_lane::<7>(vtot0) as usize;
    tot += wasm32::u8x16_extract_lane::<8>(vtot0) as usize;
    tot += wasm32::u8x16_extract_lane::<9>(vtot0) as usize;
    tot += wasm32::u8x16_extract_lane::<10>(vtot0) as usize;
    tot += wasm32::u8x16_extract_lane::<11>(vtot0) as usize;
    tot += wasm32::u8x16_extract_lane::<12>(vtot0) as usize;
    tot += wasm32::u8x16_extract_lane::<13>(vtot0) as usize;
    tot += wasm32::u8x16_extract_lane::<14>(vtot0) as usize;
    tot += wasm32::u8x16_extract_lane::<15>(vtot0) as usize;

    tot += wasm32::u8x16_extract_lane::<0>(vtot1) as usize;
    tot += wasm32::u8x16_extract_lane::<1>(vtot1) as usize;
    tot += wasm32::u8x16_extract_lane::<2>(vtot1) as usize;
    tot += wasm32::u8x16_extract_lane::<3>(vtot1) as usize;
    tot += wasm32::u8x16_extract_lane::<4>(vtot1) as usize;
    tot += wasm32::u8x16_extract_lane::<5>(vtot1) as usize;
    tot += wasm32::u8x16_extract_lane::<6>(vtot1) as usize;
    tot += wasm32::u8x16_extract_lane::<7>(vtot1) as usize;
    tot += wasm32::u8x16_extract_lane::<8>(vtot1) as usize;
    tot += wasm32::u8x16_extract_lane::<9>(vtot1) as usize;
    tot += wasm32::u8x16_extract_lane::<10>(vtot1) as usize;
    tot += wasm32::u8x16_extract_lane::<11>(vtot1) as usize;
    tot += wasm32::u8x16_extract_lane::<12>(vtot1) as usize;
    tot += wasm32::u8x16_extract_lane::<13>(vtot1) as usize;
    tot += wasm32::u8x16_extract_lane::<14>(vtot1) as usize;
    tot += wasm32::u8x16_extract_lane::<15>(vtot1) as usize;

    tot
}

pub fn reads2count4k<R>(rdr: &mut R, sep: u8) -> Result<usize, io::Error>
where
    R: Read,
{
    let mut buf: [u8; 4096] = [0; 4096];
    let mut cnt: usize = 0;
    loop {
        buf.fill(0);
        match rdr.read(&mut buf) {
            Ok(0) => return Ok(cnt),
            Ok(_) => cnt += count_lines4k(&buf, sep),
            Err(e) => match e.kind() {
                ErrorKind::Interrupted => continue,
                _ => return Err(e),
            },
        }
    }
}
