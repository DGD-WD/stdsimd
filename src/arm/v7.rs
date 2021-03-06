//! ARMv7 intrinsics.
//!
//! The reference is [ARMv7-M Architecture Reference Manual (Issue
//! E.b)][armv7m].
//!
//! [armv7m]:
//! http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0403e.
//! b/index.html

pub use super::v6::*;

#[cfg(test)]
use stdsimd_test::assert_instr;

/// Count Leading Zeros.
#[inline(always)]
#[cfg_attr(test, assert_instr(clz))]
pub unsafe fn _clz_u8(x: u8) -> u8 {
    x.leading_zeros() as u8
}

/// Count Leading Zeros.
#[inline(always)]
#[cfg_attr(test, assert_instr(clz))]
pub unsafe fn _clz_u16(x: u16) -> u16 {
    x.leading_zeros() as u16
}

/// Count Leading Zeros.
#[inline(always)]
#[cfg_attr(test, assert_instr(clz))]
pub unsafe fn _clz_u32(x: u32) -> u32 {
    x.leading_zeros() as u32
}

/// Reverse the bit order.
#[inline(always)]
#[cfg_attr(test, assert_instr(rbit))]
#[cfg_attr(target_arch = "arm", target_feature = "+v7")]
pub unsafe fn _rbit_u32(x: u32) -> u32 {
    rbit_u32(x as i32) as u32
}

#[allow(dead_code)]
extern "C" {
    #[link_name = "llvm.bitreverse.i32"]
    fn rbit_u32(i: i32) -> i32;
}


#[cfg(test)]
mod tests {
    use arm::v7;

    #[test]
    fn _clz_u8() {
        unsafe {
            assert_eq!(v7::_clz_u8(0b0000_1010u8), 4u8);
        }
    }

    #[test]
    fn _clz_u16() {
        unsafe {
            assert_eq!(v7::_clz_u16(0b0000_1010u16), 12u16);
        }
    }

    #[test]
    fn _clz_u32() {
        unsafe {
            assert_eq!(v7::_clz_u32(0b0000_1010u32), 28u32);
        }
    }

    #[test]
    fn _rbit_u32() {
        unsafe {
            assert_eq!(
                v7::_rbit_u32(0b0000_1010u32),
                0b0101_0000_0000_0000_0000_0000_0000_0000u32
            );
        }
    }
}
