use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use clap::Args;
use ron::ser::PrettyConfig;

use crate::{save_data::shared::appearance::HeadMorph, save_game::SaveKind, unreal};

#[derive(Args)]
pub struct ImportHeadMorph {
    /// Save path
    save: PathBuf,
    /// Head Morph path (.ron, .me2headmorph, .me3headmorph)
    #[arg(short, long, value_name = "HEAD_MORPH")]
    input: PathBuf,
    /// New save path
    #[arg(short, long, value_name = "NEW_SAVE")]
    output: PathBuf,
}

impl ImportHeadMorph {
    pub fn execute(self) -> Result<()> {
        let ImportHeadMorph { save, input, output } = self;

        let head_morph_file = fs::read(input).context("Failed to open the head morph")?;
        let save_file = fs::read(save).context("Failed to open the save file")?;

        let head_morph = deserialize_head_morph(head_morph_file).context("Failed to parse the head morph")?;
        let mut save_game = SaveKind::deserialize(save_file).context("Failed to parse the save file")?;

        match save_game {
            SaveKind::MassEffect1Le(ref mut me1le) => me1le.save_data.player.head_morph = Some(head_morph),
            SaveKind::MassEffect1LePs4(ref mut me1le) => me1le.player.head_morph = Some(head_morph),
            SaveKind::MassEffect2(ref mut me2) => me2.player.appearance.head_morph = Some(head_morph),
            SaveKind::MassEffect2Le(ref mut me2le) => me2le.player.appearance.head_morph = Some(head_morph),
            SaveKind::MassEffect3(ref mut me3) => me3.player.appearance.head_morph = Some(head_morph),
        }

        let new_save = save_game.serialize(&output)?;

        // Backup if file exists
        if output.exists() {
            let ext = output
                .extension()
                .map(|ext| {
                    let mut ext = ext.to_owned();
                    ext.push(".bak");
                    ext
                })
                .unwrap_or_else(|| OsString::from("bak"));

            let to = Path::with_extension(&output, ext);
            fs::copy(&output, to)?;
        }
        fs::write(output, new_save).context("Failed to write the new save file")?;

        Ok(())
    }
}

fn deserialize_head_morph(file: Vec<u8>) -> Result<HeadMorph> {
    let result = if file.starts_with(b"GIBBEDMASSEFFECT2HEADMORPH") || file.starts_with(b"GIBBEDMASSEFFECT3HEADMORPH") {
        // Gibbed's head morph
        unreal::Deserializer::from_bytes(&file[31..])?
    } else {
        // TSE head morph
        ron::de::from_bytes(&file)?
    };
    Ok(result)
}

#[derive(Args)]
pub struct ExportHeadMorph {
    /// Save path
    save: PathBuf,
    /// Head Morph path
    #[arg(short, long, value_name = "HEAD_MORPH")]
    output: PathBuf,
}

impl ExportHeadMorph {
    pub fn execute(self) -> Result<()> {
        let ExportHeadMorph { save, mut output } = self;

        output = Path::with_extension(&output, "ron");

        let save_file = fs::read(save).context("Failed to open the save file")?;
        let save_game = SaveKind::deserialize(save_file).context("Failed to parse the save file")?;

        let head_morph = match save_game {
            SaveKind::MassEffect1Le(ref me1le) => &me1le.save_data.player.head_morph,
            SaveKind::MassEffect1LePs4(ref me1le) => &me1le.player.head_morph,
            SaveKind::MassEffect2(ref me2) => &me2.player.appearance.head_morph,
            SaveKind::MassEffect2Le(ref me2le) => &me2le.player.appearance.head_morph,
            SaveKind::MassEffect3(ref me3) => &me3.player.appearance.head_morph,
        }
        .as_ref()
        .context("This save file use the default Shepard (no head morph)")?;

        let pretty_config = PrettyConfig::new().enumerate_arrays(true).new_line(String::from('\n'));
        let new_head_morph = ron::ser::to_string_pretty(head_morph, pretty_config)?;

        // Backup if file exists
        if output.exists() {
            let ext = output
                .extension()
                .map(|ext| {
                    let mut ext = ext.to_owned();
                    ext.push(".bak");
                    ext
                })
                .unwrap_or_else(|| OsString::from("bak"));

            let to = Path::with_extension(&output, ext);
            fs::copy(&output, to)?;
        }
        fs::write(output, new_head_morph).context("Failed to write the head morph")?;

        Ok(())
    }
}
