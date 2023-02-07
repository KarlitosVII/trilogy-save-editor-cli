use std::path::Path;

use anyhow::{bail, Result};
use crc::{Crc, CRC_32_BZIP2};
use serde::Deserialize;

use crate::{
    save_data::{
        mass_effect_1_le::Me1LeMagicNumber,
        mass_effect_1_le::{Me1LeSaveData, Me1LeSaveGame, Me1LeVersion},
        mass_effect_2::{Me2LeSaveGame, Me2LeVersion, Me2SaveGame, Me2Version},
        mass_effect_3::{Me3SaveGame, Me3Version},
    },
    unreal,
};

#[derive(From)]
pub enum SaveKind {
    MassEffect1Le(Box<Me1LeSaveGame>),
    MassEffect1LePs4(Box<Me1LeSaveData>),
    MassEffect2(Box<Me2SaveGame>),
    MassEffect2Le(Box<Me2LeSaveGame>),
    MassEffect3(Box<Me3SaveGame>),
}

impl SaveKind {
    pub fn deserialize(input: Vec<u8>) -> Result<Self> {
        fn header<'de, T>(header: &'de [u8]) -> Result<T, unreal::Error>
        where
            T: Deserialize<'de>,
        {
            unreal::Deserializer::from_bytes::<T>(header)
        }

        let this = if header::<Me1LeMagicNumber>(&input).is_ok() {
            // ME1 Legendary
            let me1_le: Me1LeSaveGame = unreal::Deserializer::from_bytes(&input)?;
            Box::new(me1_le).into()
        } else if header::<Me1LeVersion>(&input).is_ok() {
            // ME1LE PS4
            let me1_le_ps4: Me1LeSaveData = unreal::Deserializer::from_bytes(&input)?;
            Box::new(me1_le_ps4).into()
        } else if let Ok(save) = header::<Me2Version>(&input) {
            // ME2
            let me2: Me2SaveGame = if save.is_xbox360 {
                unreal::Deserializer::from_be_bytes(&input)?
            } else {
                unreal::Deserializer::from_bytes(&input)?
            };
            Box::new(me2).into()
        } else if header::<Me2LeVersion>(&input).is_ok() {
            // ME2 Legendary
            let me2_le: Me2LeSaveGame = unreal::Deserializer::from_bytes(&input)?;
            Box::new(me2_le).into()
        } else if let Ok(save) = header::<Me3Version>(&input) {
            // ME3
            let me3: Me3SaveGame = if save.is_xbox360 {
                unreal::Deserializer::from_be_bytes(&input)?
            } else {
                unreal::Deserializer::from_bytes(&input)?
            };
            Box::new(me3).into()
        } else {
            bail!("Unsupported file");
        };

        Ok(this)
    }

    pub fn serialize(&self, path: &Path) -> Result<Vec<u8>> {
        let output = match self {
            SaveKind::MassEffect1Le(ref save_data) => {
                let mut output = unreal::Serializer::to_vec(save_data)?;

                // Checksum
                let checksum_offset = output.len() - 12;
                let crc = Crc::<u32>::new(&CRC_32_BZIP2);
                let checksum = crc.checksum(&output[..checksum_offset]);

                // Update checksum
                let end = checksum_offset + 4;
                output[checksum_offset..end].swap_with_slice(&mut u32::to_le_bytes(checksum));
                output
            }
            SaveKind::MassEffect1LePs4(ref save_data) => unreal::Serializer::to_vec(save_data)?,
            SaveKind::MassEffect2(ref save_data) => {
                let is_xbox360 = path
                    .extension()
                    .map(|ext| ext.eq_ignore_ascii_case("xbsav"))
                    .unwrap_or_default();

                let mut output = if is_xbox360 {
                    unreal::Serializer::to_be_vec(save_data)?
                } else {
                    unreal::Serializer::to_vec(save_data)?
                };

                let crc = Crc::<u32>::new(&CRC_32_BZIP2);
                let checksum = crc.checksum(&output);

                let extend = if is_xbox360 {
                    u32::to_be_bytes(checksum)
                } else {
                    u32::to_le_bytes(checksum)
                };
                output.extend(extend);
                output
            }
            SaveKind::MassEffect2Le(ref save_data) => {
                let mut output = unreal::Serializer::to_vec(save_data)?;

                let crc = Crc::<u32>::new(&CRC_32_BZIP2);
                let checksum = crc.checksum(&output);
                output.extend(&u32::to_le_bytes(checksum));
                output
            }
            SaveKind::MassEffect3(ref save_data) => {
                let is_xbox360 = path
                    .extension()
                    .map(|ext| ext.eq_ignore_ascii_case("xbsav"))
                    .unwrap_or_default();

                let mut output = if is_xbox360 {
                    unreal::Serializer::to_be_vec(save_data)?
                } else {
                    unreal::Serializer::to_vec(save_data)?
                };

                let crc = Crc::<u32>::new(&CRC_32_BZIP2);
                let checksum = crc.checksum(&output);

                let extend = if is_xbox360 {
                    u32::to_be_bytes(checksum)
                } else {
                    u32::to_le_bytes(checksum)
                };
                output.extend(extend);
                output
            }
        };

        Ok(output)
    }
}
