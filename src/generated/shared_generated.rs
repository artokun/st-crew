// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod shared {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

// struct Vec2, aligned to 4
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Vec2(pub [u8; 8]);
impl Default for Vec2 { 
  fn default() -> Self { 
    Self([0; 8])
  }
}
impl core::fmt::Debug for Vec2 {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    f.debug_struct("Vec2")
      .field("x", &self.x())
      .field("y", &self.y())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Vec2 {}
impl<'a> flatbuffers::Follow<'a> for Vec2 {
  type Inner = &'a Vec2;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Vec2>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Vec2 {
  type Inner = &'a Vec2;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Vec2>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Vec2 {
    type Output = Vec2;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        let src = ::core::slice::from_raw_parts(self as *const Vec2 as *const u8, Self::size());
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for Vec2 {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}

impl<'a> Vec2 {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    x: f32,
    y: f32,
  ) -> Self {
    let mut s = Self([0; 8]);
    s.set_x(x);
    s.set_y(y);
    s
  }

  pub fn x(&self) -> f32 {
    let mut mem = core::mem::MaybeUninit::<<f32 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_x(&mut self, x: f32) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn y(&self) -> f32 {
    let mut mem = core::mem::MaybeUninit::<<f32 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[4..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_y(&mut self, x: f32) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[4..].as_mut_ptr(),
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
    }
  }

}

}  // pub mod shared

