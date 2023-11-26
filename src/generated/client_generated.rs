// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod client {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_CLIENT_ACTION: i8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_CLIENT_ACTION: i8 = 1;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_CLIENT_ACTION: [ClientAction; 2] = [
  ClientAction::Joined,
  ClientAction::Left,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ClientAction(pub i8);
#[allow(non_upper_case_globals)]
impl ClientAction {
  pub const Joined: Self = Self(0);
  pub const Left: Self = Self(1);

  pub const ENUM_MIN: i8 = 0;
  pub const ENUM_MAX: i8 = 1;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::Joined,
    Self::Left,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::Joined => Some("Joined"),
      Self::Left => Some("Left"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for ClientAction {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for ClientAction {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<i8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for ClientAction {
    type Output = ClientAction;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<i8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for ClientAction {
  type Scalar = i8;
  #[inline]
  fn to_little_endian(self) -> i8 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: i8) -> Self {
    let b = i8::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for ClientAction {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    i8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for ClientAction {}
pub enum ClientOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Client<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Client<'a> {
  type Inner = Client<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Client<'a> {
  pub const VT_ID: flatbuffers::VOffsetT = 4;
  pub const VT_NAME: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Client { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args ClientArgs<'args>
  ) -> flatbuffers::WIPOffset<Client<'bldr>> {
    let mut builder = ClientBuilder::new(_fbb);
    if let Some(x) = args.name { builder.add_name(x); }
    if let Some(x) = args.id { builder.add_id(x); }
    builder.finish()
  }


  #[inline]
  pub fn id(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Client::VT_ID, None)}
  }
  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Client::VT_NAME, None)}
  }
}

impl flatbuffers::Verifiable for Client<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("id", Self::VT_ID, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("name", Self::VT_NAME, false)?
     .finish();
    Ok(())
  }
}
pub struct ClientArgs<'a> {
    pub id: Option<flatbuffers::WIPOffset<&'a str>>,
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for ClientArgs<'a> {
  #[inline]
  fn default() -> Self {
    ClientArgs {
      id: None,
      name: None,
    }
  }
}

pub struct ClientBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ClientBuilder<'a, 'b> {
  #[inline]
  pub fn add_id(&mut self, id: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Client::VT_ID, id);
  }
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Client::VT_NAME, name);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ClientBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ClientBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Client<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Client<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Client");
      ds.field("id", &self.id());
      ds.field("name", &self.name());
      ds.finish()
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `Client`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_client_unchecked`.
pub fn root_as_client(buf: &[u8]) -> Result<Client, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<Client>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `Client` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_client_unchecked`.
pub fn size_prefixed_root_as_client(buf: &[u8]) -> Result<Client, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<Client>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `Client` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_client_unchecked`.
pub fn root_as_client_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Client<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<Client<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `Client` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_client_unchecked`.
pub fn size_prefixed_root_as_client_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Client<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<Client<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a Client and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `Client`.
pub unsafe fn root_as_client_unchecked(buf: &[u8]) -> Client {
  flatbuffers::root_unchecked::<Client>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed Client and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `Client`.
pub unsafe fn size_prefixed_root_as_client_unchecked(buf: &[u8]) -> Client {
  flatbuffers::size_prefixed_root_unchecked::<Client>(buf)
}
#[inline]
pub fn finish_client_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Client<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_client_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<Client<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod client

