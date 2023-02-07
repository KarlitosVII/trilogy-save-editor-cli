pub mod legacy;
pub mod player;
pub mod squad;

use self::{legacy::*, player::*, squad::*};

use std::{fmt, io::Read};

use anyhow::Result;
use flate2::{
    read::{ZlibDecoder, ZlibEncoder},
    Compression,
};
use serde::{
    de,
    ser::{self, SerializeStruct},
    {Deserialize, Deserializer, Serialize, Serializer},
};

use crate::{
    save_data::{
        shared::{
            plot::{Codex, Journal, PlotTable},
            Rotator, SaveTimeStamp, Vector3D,
        },
        List,
    },
    unreal, IndexMap,
};

#[derive(Serialize, Clone)]
struct ChunkHeader {
    compressed_size: u32,
    uncompressed_size: u32,
}

pub struct Me1LeSaveGame {
    magic_number: Me1LeMagicNumber,
    block_size: u32,
    _headers: List<ChunkHeader>,
    pub save_data: Me1LeSaveData,
    checksum: u32,
    compression_flag: u32, // 1 = ZLIB
    _uncompressed_size: u32,
}

impl<'de> Deserialize<'de> for Me1LeSaveGame {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Me1LeSaveGameVisitor;
        impl<'de> de::Visitor<'de> for Me1LeSaveGameVisitor {
            type Value = Me1LeSaveGame;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a Me1LeSaveGame")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let magic_number = seq.next_element()?.unwrap();
                let block_size = seq.next_element()?.unwrap();

                // Headers
                let mut headers = Vec::new();
                {
                    let full_header = ChunkHeader {
                        compressed_size: seq.next_element()?.unwrap(),
                        uncompressed_size: seq.next_element()?.unwrap(),
                    };
                    headers.push(full_header);

                    let mut finished = false;
                    while !finished {
                        let header = ChunkHeader {
                            compressed_size: seq.next_element()?.unwrap(),
                            uncompressed_size: seq.next_element()?.unwrap(),
                        };
                        if header.uncompressed_size < block_size {
                            finished = true;
                        }
                        headers.push(header);
                    }
                }

                // Save data
                let save_data: Me1LeSaveData = {
                    let mut uncompressed = Vec::new();

                    for header in &headers[1..] {
                        let mut compressed = Vec::new();
                        for _ in 0..header.compressed_size {
                            compressed.push(seq.next_element()?.unwrap());
                        }

                        let mut z = ZlibDecoder::new(&compressed[..]);
                        z.read_to_end(&mut uncompressed).map_err(de::Error::custom)?;
                    }

                    unreal::Deserializer::from_bytes(&uncompressed).map_err(de::Error::custom)?
                };

                let checksum = seq.next_element()?.unwrap();
                let compression_flag = seq.next_element()?.unwrap();
                let uncompressed_size = seq.next_element()?.unwrap();

                Ok(Me1LeSaveGame {
                    magic_number,
                    block_size,
                    _headers: headers.into(),
                    save_data,
                    checksum,
                    compression_flag,
                    _uncompressed_size: uncompressed_size,
                })
            }
        }
        deserializer.deserialize_tuple_struct("Me1LeSaveGame", usize::MAX, Me1LeSaveGameVisitor)
    }
}

impl serde::Serialize for Me1LeSaveGame {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let Me1LeSaveGame {
            magic_number,
            block_size,
            _headers,
            save_data,
            checksum,
            compression_flag,
            _uncompressed_size,
        } = self;

        let mut headers = Vec::new();

        let uncompressed = unreal::Serializer::to_vec(save_data).map_err(ser::Error::custom)?;

        headers.push(ChunkHeader {
            compressed_size: 0,
            uncompressed_size: uncompressed.len() as u32,
        });

        // Compresse chaque chunk
        let mut compressed = Vec::new();
        for chunk in uncompressed.chunks(*block_size as usize) {
            let uncompressed_size = chunk.len() as u32;

            let mut compressed_chunk = Vec::new();
            {
                let mut z = ZlibEncoder::new(chunk, Compression::default());
                z.read_to_end(&mut compressed_chunk).map_err(ser::Error::custom)?;
            }

            let compressed_size = compressed_chunk.len() as u32;

            headers[0].compressed_size += compressed_size;
            headers.push(ChunkHeader {
                compressed_size,
                uncompressed_size,
            });

            compressed.extend(&compressed_chunk);
        }
        let headers: List<_> = headers.into();
        let save_data: List<u8> = compressed.into();

        let mut s = serializer.serialize_struct("Me1LeSaveGame", 4)?;
        s.serialize_field("magic_number", magic_number)?;
        s.serialize_field("block_size", block_size)?;
        s.serialize_field("headers", &headers)?;
        s.serialize_field("save_data", &save_data)?;
        s.serialize_field("checksum", checksum)?;
        s.serialize_field("compression_flag", compression_flag)?;
        s.serialize_field("uncompressed_size", &headers[0].uncompressed_size)?;
        s.end()
    }
}

#[derive(Serialize)]
pub struct Me1LeMagicNumber(u32);

impl<'de> Deserialize<'de> for Me1LeMagicNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let version: [u8; 4] = Deserialize::deserialize(deserializer)?;

        if version != [0xC1, 0x83, 0x2A, 0x9E] {
            return Err(de::Error::custom("Wrong magic number"));
        }

        Ok(Self(u32::from_le_bytes(version)))
    }
}

#[derive(Deserialize, Serialize)]
pub struct Me1LeSaveData {
    _version: Me1LeVersion,
    character_id: String,
    created_date: SaveTimeStamp,
    pub plot: PlotTable,
    journal: Journal,
    codex: Codex,
    timestamp: SaveTimeStamp,
    seconds_played: i32,
    pub player: Player,
    base_level_name: String,
    map_name: String,
    parent_map_name: String,
    location: Vector3D,
    rotation: Rotator,
    pub squad: Vec<Henchman>,
    display_name: String,
    file_name: String,
    pub no_export: NoExport, // Only serialized for normal savegames, not for character export
}

#[derive(Serialize)]
pub struct Me1LeVersion(i32);

impl<'de> Deserialize<'de> for Me1LeVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let version: i32 = Deserialize::deserialize(deserializer)?;

        if version != 50 {
            return Err(de::Error::custom(
                "Wrong save version, please use a save from the latest version of the game",
            ));
        }

        Ok(Self(version))
    }
}

pub struct NoExport(Option<NoExportData>);

// impl NoExport {
//     pub fn as_ref(&self) -> Option<&NoExportData> {
//         self.0.as_ref()
//     }
// }

impl<'de> Deserialize<'de> for NoExport {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NoExportVisitor;
        impl<'de> de::Visitor<'de> for NoExportVisitor {
            type Value = NoExport;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a NoExport")
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                let no_export_data: NoExportData = Deserialize::deserialize(deserializer)?;
                Ok(NoExport(Some(no_export_data)))
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(NoExport(None))
            }
        }
        deserializer.deserialize_any(NoExportVisitor)
    }
}

impl serde::Serialize for NoExport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0 {
            Some(ref no_export_data) => no_export_data.serialize(serializer),
            None => serializer.serialize_unit(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct NoExportData {
    legacy_maps: IndexMap<String, Map>,
    mako: Vehicle,
}

#[derive(Deserialize, Serialize)]
pub struct Vehicle {
    first_name: String,
    localized_last_name: i32,
    health: f32,
    shield: f32,
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::time::Instant;

    use anyhow::Result;
    use crc::{Crc, CRC_32_BZIP2};

    use super::*;
    use crate::unreal;

    #[test]
    fn deserialize_serialize() -> Result<()> {
        let files = [
            "test/ME1LeSave.pcsav",   // Normal save game
            "test/ME1LeExport.pcsav", // Export save game
        ];

        for file in files {
            let input = fs::read(file)?;

            let now = Instant::now();

            // Deserialize
            let me1_save_game: Me1LeSaveGame = unreal::Deserializer::from_bytes(&input)?;

            println!("Deserialize 1 : {:?}", Instant::now() - now);
            let now = Instant::now();

            // Serialize
            let mut output = unreal::Serializer::to_vec(&me1_save_game)?;

            // Checksum
            {
                let checksum_offset = output.len() - 12;
                let crc = Crc::<u32>::new(&CRC_32_BZIP2);
                let checksum = crc.checksum(&output[..checksum_offset]);

                // Update checksum
                let end = checksum_offset + 4;
                output[checksum_offset..end].swap_with_slice(&mut u32::to_le_bytes(checksum));
            }

            println!("Serialize 1 : {:?}", Instant::now() - now);
            let now = Instant::now();

            // Deserialize (again)
            let me1_save_game: Me1LeSaveGame = unreal::Deserializer::from_bytes(&output)?;

            println!("Deserialize 2 : {:?}", Instant::now() - now);
            let now = Instant::now();

            // Serialize (again)
            let mut output_2 = unreal::Serializer::to_vec(&me1_save_game)?;

            // Checksum
            {
                let checksum_offset = output_2.len() - 12;
                let crc = Crc::<u32>::new(&CRC_32_BZIP2);
                let checksum = crc.checksum(&output_2[..checksum_offset]);

                // Update checksum
                let end = checksum_offset + 4;
                output_2[checksum_offset..end].swap_with_slice(&mut u32::to_le_bytes(checksum));
            }

            println!("Serialize 2 : {:?}", Instant::now() - now);

            // Check 2nd serialize = first serialize
            // let cmp = output.chunks(4).zip(output_2.chunks(4));
            // for (i, (a, b)) in cmp.enumerate() {
            //     if a != b {
            //         panic!("0x{:02x?} : {:02x?} != {:02x?}", i * 4, a, b);
            //     }
            // }

            // Check 2nd serialize = first serialize
            assert!(output == output_2);
        }
        Ok(())
    }

    // #[test]
    // fn uncompress() -> Result<()> {
    //     let input = fs::read("test/ME1Le_Export.pcsav")?;
    //     let me1_save_game: Me1LeSaveGame = unreal::Deserializer::from_bytes(&input)?;

    //     let output = unreal::Serializer::to_vec(&me1_save_game.save_data)?;
    //     fs::write("test/ME1Le_Export.uncompressed", &output)?;

    //     Ok(())
    // }
}
