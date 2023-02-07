pub mod mass_effect_1_le;
pub mod mass_effect_2;
pub mod mass_effect_3;
pub mod shared;

use std::fmt;

use anyhow::Result;
use serde::{de, ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

// Implémentation des dummy
#[derive(Clone)]
pub struct Dummy<const BYTE_LEN: usize>([u8; BYTE_LEN]);

impl<const BYTE_LEN: usize> Default for Dummy<BYTE_LEN> {
    fn default() -> Self {
        Self([0; BYTE_LEN])
    }
}

impl<'de, const BYTE_LEN: usize> Deserialize<'de> for Dummy<BYTE_LEN> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DummyVisitor<const BYTE_LEN: usize>;
        impl<'de, const BYTE_LEN: usize> de::Visitor<'de> for DummyVisitor<BYTE_LEN> {
            type Value = Dummy<BYTE_LEN>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a Dummy<BYTE_LEN>")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut result = [0_u8; BYTE_LEN];
                let mut i = 0;
                while let Some(element) = seq.next_element()? {
                    result[i] = element;
                    i += 1;
                }
                Ok(Dummy(result))
            }
        }
        deserializer.deserialize_tuple_struct("Dummy<BYTE_LEN>", BYTE_LEN, DummyVisitor)
    }
}

impl<const BYTE_LEN: usize> serde::Serialize for Dummy<BYTE_LEN> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.0)
    }
}

// List<T> : Vec<T> qui se (dé)sérialise sans précision de longueur
#[derive(Deref, DerefMut, From, Clone)]
pub struct List<T>(Vec<T>)
where
    T: Serialize + Clone;

impl<T> From<&[T]> for List<T>
where
    T: Serialize + Clone,
{
    fn from(from: &[T]) -> List<T> {
        List(from.to_vec())
    }
}

impl<'de> Deserialize<'de> for List<u8> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ByteListVisitor;
        impl<'de> de::Visitor<'de> for ByteListVisitor {
            type Value = List<u8>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a byte buf")
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(List(v))
            }
        }
        deserializer.deserialize_byte_buf(ByteListVisitor)
    }
}

impl<T> serde::Serialize for List<T>
where
    T: Serialize + Clone,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_seq(None)?;
        for element in &self.0 {
            s.serialize_element(element)?;
        }
        s.end()
    }
}

#[derive(Clone, From, Default)]
pub struct Guid(Uuid);

// impl Guid {
//     pub fn hyphenated(&self) -> String {
//         self.0.as_hyphenated().to_string()
//     }
// }

impl<'de> Deserialize<'de> for Guid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (d1, d2, d3, d4): (u32, u16, u16, [u8; 8]) = Deserialize::deserialize(deserializer)?;
        Ok(Guid(Uuid::from_fields(d1, d2, d3, &d4)))
    }
}

impl serde::Serialize for Guid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serde::Serialize::serialize(&self.0.as_fields(), serializer)
    }
}
