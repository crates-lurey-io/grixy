#[cfg(feature = "serde")]
extern crate alloc;

#[cfg(feature = "serde")]
use crate::buf::GridBuf;
#[cfg(feature = "serde")]
use crate::ops::layout;
#[cfg(feature = "serde")]
use core::marker::PhantomData;
#[cfg(feature = "serde")]
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Visitor},
};

#[cfg(feature = "serde")]
impl<T, B, L> Serialize for GridBuf<T, B, L>
where
    T: Serialize,
    B: AsRef<[T]>,
    L: layout::Linear,
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("GridBuf", 3)?;
        state.serialize_field("width", &self.width)?;
        state.serialize_field("height", &self.height)?;
        state.serialize_field("data", self.buffer.as_ref())?;
        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de, T, L> Deserialize<'de> for GridBuf<T, alloc::vec::Vec<T>, L>
where
    T: Deserialize<'de> + Default + Copy,
    L: layout::Linear,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        enum Field {
            Width,
            Height,
            Data,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                        f.write_str("`width`, `height`, or `data`")
                    }

                    fn visit_str<E: de::Error>(self, value: &str) -> Result<Field, E> {
                        match value {
                            "width" => Ok(Field::Width),
                            "height" => Ok(Field::Height),
                            "data" => Ok(Field::Data),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        const FIELDS: &[&str] = &["width", "height", "data"];

        struct GridBufVisitor<T, L>(PhantomData<(T, L)>);

        impl<'de, T, L> Visitor<'de> for GridBufVisitor<T, L>
        where
            T: Deserialize<'de> + Default + Copy,
            L: layout::Linear,
        {
            type Value = GridBuf<T, alloc::vec::Vec<T>, L>;

            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.write_str("struct GridBuf")
            }

            fn visit_map<V: de::MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let mut width = None;
                let mut height = None;
                let mut data = None;

                while let Some(key) = map.next_key::<Field>()? {
                    match key {
                        Field::Width => {
                            if width.is_some() {
                                return Err(de::Error::duplicate_field("width"));
                            }
                            width = Some(map.next_value::<usize>()?);
                        }
                        Field::Height => {
                            if height.is_some() {
                                return Err(de::Error::duplicate_field("height"));
                            }
                            height = Some(map.next_value::<usize>()?);
                        }
                        Field::Data => {
                            if data.is_some() {
                                return Err(de::Error::duplicate_field("data"));
                            }
                            data = Some(map.next_value::<alloc::vec::Vec<T>>()?);
                        }
                    }
                }

                let width = width.ok_or_else(|| de::Error::missing_field("width"))?;
                let height = height.ok_or_else(|| de::Error::missing_field("height"))?;
                let data = data.ok_or_else(|| de::Error::missing_field("data"))?;

                if data.len() != width * height {
                    return Err(de::Error::custom(alloc::format!(
                        "data length {} does not match width {} * height {}",
                        data.len(),
                        width,
                        height
                    )));
                }

                Ok(GridBuf {
                    buffer: data,
                    width,
                    height,
                    _layout: PhantomData,
                    _element: PhantomData,
                })
            }
        }

        deserializer.deserialize_struct("GridBuf", FIELDS, GridBufVisitor::<T, L>(PhantomData))
    }
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod tests {
    extern crate alloc;

    use crate::{
        buf::GridBuf,
        core::Pos,
        ops::{ExactSizeGrid as _, GridRead as _, layout::RowMajor},
    };

    #[test]
    fn serde_roundtrip() {
        let mut grid = GridBuf::<_, _, RowMajor>::new_filled(3, 3, 0u8);
        grid[Pos::new(1, 1)] = 42;

        let json = serde_json::to_string(&grid).unwrap();
        let deserialized: GridBuf<u8, alloc::vec::Vec<u8>, RowMajor> =
            serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.width(), 3);
        assert_eq!(deserialized.height(), 3);
        assert_eq!(deserialized.get(Pos::new(1, 1)), Some(&42));
        assert_eq!(deserialized.get(Pos::new(0, 0)), Some(&0));
    }
}
