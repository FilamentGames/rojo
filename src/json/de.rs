use serde_json::de::Read;
use serde_json::Error;
use serde_json::Result;

// FIXME: None of this works yet

pub struct Deserializer<'r, R> {
    read: &'r mut R,
    scratch: Vec<u8>,
    json_deserializer: serde_json::Deserializer<R>,
}

impl<'de, 'r, R> Deserializer<'r, R>
where
    R: Read<'de>,
{
    pub fn new(mut read: R) -> Self {
        Deserializer {
            scratch: Vec::new(),
            read: &mut read,
            json_deserializer: serde_json::Deserializer::new(read),
        }
    }
}

impl<'de, 'r, R: Read<'de>> Deserializer<'r, R> {
    pub fn end(&mut self) -> Result<()> {
        self.json_deserializer.end()
    }

    fn peek(&mut self) -> Result<Option<u8>> {
        self.read.peek()
    }

    fn parse_whitespace(&mut self) -> Result<Option<u8>> {
        loop {
            match self.peek()? {
                Some(b' ') | Some(b'\n') | Some(b'\t') | Some(b'\r') => {
                    self.eat_char();
                }
                other => {
                    return Ok(other);
                }
            }
        }
    }

    fn eat_char(&mut self) {
        self.read.discard();
    }
}

#[derive(std::fmt::Debug)]
struct DummyError {}

impl std::fmt::Display for DummyError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl std::error::Error for DummyError {}

impl<'de, 'a, 'r, R: Read<'de>> serde::Deserializer<'de> for &'a mut Deserializer<'r, R> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer.deserialize_any(visitor)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer.deserialize_bool(visitor)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer.deserialize_i8(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer.deserialize_i16(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer.deserialize_i32(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer.deserialize_i64(visitor)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer.deserialize_u8(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer.deserialize_u16(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer.deserialize_u32(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer.deserialize_u64(visitor)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let peek = match self.parse_whitespace()? {
            Some(b) => b,
            None => {
                return Err(Error::io(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    DummyError {},
                )));
            }
        };

        let value = match peek {
            b'"' => {
                self.eat_char();
                self.scratch.clear();
                let value = self.read.parse_str(&mut self.scratch)?;

                match &*value {
                    "NaN" => return visitor.visit_f32(f32::NAN),
                    "Infinity" => return visitor.visit_f32(f32::INFINITY),
                    "-Infinity" => return visitor.visit_f32(f32::NEG_INFINITY),
                    _ => return Err(Error::io(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        DummyError {},
                    ))),
                };
            }
            _ => return self.json_deserializer.deserialize_f32(visitor),
        };
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let peek = match self.parse_whitespace()? {
            Some(b) => b,
            None => {
                return Err(Error::io(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    DummyError {},
                )));
            }
        };

        let value = match peek {
            b'"' => {
                self.eat_char();
                self.scratch.clear();
                let value = self.read.parse_str(&mut self.scratch)?;

                match &*value {
                    "NaN" => return visitor.visit_f64(f64::NAN),
                    "Infinity" => return visitor.visit_f64(f64::INFINITY),
                    "-Infinity" => return visitor.visit_f64(f64::NEG_INFINITY),
                    _ => return Err(Error::io(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        DummyError {},
                    ))),
                };
            }
            _ => return self.json_deserializer.deserialize_f64(visitor),
        };
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer.deserialize_char(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer.deserialize_str(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer.deserialize_string(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer.deserialize_bytes(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer.deserialize_byte_buf(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer.deserialize_option(visitor)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer.deserialize_unit(visitor)
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer
            .deserialize_unit_struct(name, visitor)
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer
            .deserialize_newtype_struct(name, visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer.deserialize_seq(visitor)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer.deserialize_tuple(len, visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer
            .deserialize_tuple_struct(name, len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer.deserialize_map(visitor)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer
            .deserialize_struct(name, fields, visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer
            .deserialize_enum(name, variants, visitor)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer.deserialize_identifier(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.json_deserializer.deserialize_ignored_any(visitor)
    }
}

fn from_trait<'de, R, T>(read: R) -> Result<T>
where
    R: Read<'de>,
    T: serde::Deserialize<'de>,
{
    let mut de = Deserializer::new(read);
    let value = serde::de::Deserialize::deserialize(&mut de)?;

    // Make sure the whole stream has been consumed.
    de.end();
    Ok(value)
}

pub fn from_slice<'a, T>(v: &'a [u8]) -> Result<T>
where
    T: serde::Deserialize<'a>,
{
    from_trait(serde_json::de::SliceRead::new(v))
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: serde::Deserialize<'a>,
{
    from_trait(serde_json::de::StrRead::new(s))
}
