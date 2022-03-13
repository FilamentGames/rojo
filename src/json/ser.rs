
use serde_json::{
    Error,
    Result,
    ser::{
        Compound,
        Formatter,
        CompactFormatter,
    }
};

use serde::Serialize;

use std::num::FpCategory;
use std::io;

#[inline]
pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
where
    W: io::Write,
    T: ?Sized + Serialize,
{
    let mut ser = Serializer::new(writer, CompactFormatter {});
    value.serialize(&mut ser);
    Ok(())
}

#[inline]
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value);
    Ok(writer)
}

#[inline]
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: ?Sized + Serialize,
{
    let vec = to_vec(value)?;
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}

pub struct Serializer<W, F> {
    json_serializer: serde_json::Serializer<W, F>,
}

impl<W, F> Serializer<W, F>
where
    W: io::Write,
    F: Formatter,
{
    /// Creates a new JSON serializer.
    #[inline]
    pub fn new(writer: W, formatter: F) -> Self {
        Serializer { json_serializer: serde_json::Serializer::with_formatter(writer, formatter) }
    }
}


impl<'a, W, F> serde::ser::Serializer for &'a mut Serializer<W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Compound<'a, W, F>;
    type SerializeTuple = Compound<'a, W, F>;
    type SerializeTupleStruct = Compound<'a, W, F>;
    type SerializeTupleVariant = Compound<'a, W, F>;
    type SerializeMap = Compound<'a, W, F>;
    type SerializeStruct = Compound<'a, W, F>;
    type SerializeStructVariant = Compound<'a, W, F>;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.json_serializer.serialize_bool(v)
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.json_serializer.serialize_i8(v)
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.json_serializer.serialize_i16(v)
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.json_serializer.serialize_i32(v)
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.json_serializer.serialize_i64(v)
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.json_serializer.serialize_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.json_serializer.serialize_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.json_serializer.serialize_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.json_serializer.serialize_u64(v)
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        match v.classify() {
            FpCategory::Nan => {
                self.json_serializer.serialize_str("NaN")
            }
            FpCategory::Infinite => {
                if v == f32::INFINITY {
                    self.json_serializer.serialize_str("Infinity")
                } else {
                    self.json_serializer.serialize_str("-Infinity")
                }
            }
            _ => {
                self.json_serializer.serialize_f32(v)
            }
        }
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        match v.classify() {
            FpCategory::Nan => {
                self.json_serializer.serialize_str("NaN")
            }
            FpCategory::Infinite => {
                if v == f64::INFINITY {
                    self.json_serializer.serialize_str("Infinity")
                } else {
                    self.json_serializer.serialize_str("-Infinity")
                }
            }
            _ => {
                self.json_serializer.serialize_f64(v)
            }
        }
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.json_serializer.serialize_char(v)
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.json_serializer.serialize_str(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.json_serializer.serialize_bytes(v)
    }

    fn serialize_none(self) -> Result<()> {
        self.json_serializer.serialize_none()
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        self.json_serializer.serialize_some(value)
    }

    fn serialize_unit(self) -> Result<()> {
        self.json_serializer.serialize_unit()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.json_serializer.serialize_unit_struct(_name)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.json_serializer.serialize_unit_variant(_name, _variant_index, variant)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        self.json_serializer.serialize_newtype_struct(_name, value)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        self.json_serializer.serialize_newtype_variant(_name, _variant_index, variant, value)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.json_serializer.serialize_seq(_len)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        self.json_serializer.serialize_tuple(_len)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.json_serializer.serialize_tuple_struct(_name, _len)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.json_serializer.serialize_tuple_variant(
            _name,
            _variant_index,
            variant,
            _len,
        )
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.json_serializer.serialize_map(_len)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        self.json_serializer.serialize_struct(_name, _len)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.json_serializer.serialize_struct_variant(
            _name,
            _variant_index,
            variant,
            _len,
        )
    }
}
