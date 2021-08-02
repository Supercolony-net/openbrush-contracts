use cargo_metadata::MetadataCommand;
use fs2::FileExt;
use proc_macro2::TokenStream as TokenStream2;
use serde::{
    Deserialize,
    Serialize,
};
use serde_json;
use std::{
    collections::HashMap,
    env,
    fs::{
        File,
        OpenOptions,
    },
    io::{
        BufReader,
        Seek,
        SeekFrom,
    },
    path::PathBuf,
    str::FromStr,
};
use syn::{
    ItemTrait,
    TraitItem,
};
use unwrap::unwrap;

const TEMP_FILE: &'static str = "__brush_metadata";

#[derive(Default, Debug, Serialize, Deserialize)]
pub(crate) struct TraitDefinitions(HashMap<String, String>);

impl std::ops::Deref for TraitDefinitions {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for TraitDefinitions {
    fn deref_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.0
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub(crate) struct ModifierDefinitions(HashMap<String, String>);

impl std::ops::Deref for ModifierDefinitions {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ModifierDefinitions {
    fn deref_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.0
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub(crate) struct Metadata {
    pub external_traits: TraitDefinitions,
}

impl Metadata {
    pub(crate) fn load(file: &File) -> Metadata {
        let reader = BufReader::new(file);

        let map = serde_json::from_reader(reader).unwrap_or_default();
        map
    }

    pub(crate) fn save_and_unlock(&self, mut locked_file: File) {
        locked_file.set_len(0).expect("Can't truncate the file");
        locked_file.seek(SeekFrom::Start(0)).expect("Can't set cursor position");
        serde_json::to_writer(&locked_file, self).expect("Can't dump definition metadata to file");
        locked_file.unlock().expect("Can't remove exclusive lock");
    }
}

pub(crate) struct TraitDefinition(ItemTrait);

impl TraitDefinition {
    pub(crate) fn methods(&self) -> Vec<syn::TraitItemMethod> {
        self.0
            .items
            .clone()
            .into_iter()
            .filter_map(|item| {
                if let TraitItem::Method(method) = item {
                    Some(method)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl TraitDefinitions {
    pub(crate) fn get(&self, ident: &String) -> TraitDefinition {
        let stream = unwrap!(
            TokenStream2::from_str(unwrap!(self.0.get(ident), "Can't find definition of trait {}", ident)),
            "Trait definition({}) is not TokenStream",
            ident
        );
        let trait_item = unwrap!(syn::parse2::<ItemTrait>(stream), "Can't parse ItemTrait of {}", ident);

        TraitDefinition { 0: trait_item }
    }
}

/// Function returns exclusively locked file for metadata.
/// It stores file in the nearest target folder
/// from the directory where the build command has been invoked(output of `pwd` command).
/// If the directory doesn't contain `Cargo.toml` file,
/// it will try to find `Cargo.toml` in the upper directories.
pub(crate) fn get_locked_file() -> File {
    let mut manifest_path = PathBuf::from(env::var("PWD").expect("Can't get PWD")).join("Cargo.toml");

    // if the current directory does not contain a Cargo.toml file, go up until you find it.
    while !manifest_path.exists() {
        if let Some(str) = manifest_path.as_os_str().to_str() {
            // If `/Cargo.toml` is not exist, it means that we will do infinity while, so break it
            assert_ne!(str, "/Cargo.toml", "Can't find Cargo.toml in directories tree");
        }
        // Remove Cargo.toml
        manifest_path.pop();
        // Remove parent folder
        manifest_path.pop();
        manifest_path = manifest_path.join("Cargo.toml");
    }

    let mut cmd = MetadataCommand::new();
    let metadata = cmd
        .manifest_path(manifest_path.clone())
        .exec()
        .expect("Error invoking `cargo metadata`");

    let dir = metadata.target_directory.join(TEMP_FILE);

    let file = match OpenOptions::new().read(true).write(true).create(true).open(&dir) {
        Err(why) => panic!("Couldn't open temporary storage: {}", why),
        Ok(file) => file,
    };
    file.lock_exclusive().expect("Can't do exclusive lock");
    file
}
