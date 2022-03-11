use cargo_metadata::camino::Utf8PathBuf;
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

pub(crate) enum LockType {
    Exclusive,
    Shared,
}

/// Function returns exclusively locked file for metadata.
/// It stores file in the target folder where `ink_lang` is stored.
pub(crate) fn get_locked_file(t: LockType) -> File {
    const PREFIX: &str = "ink_lang=";
    const SUFFIX: &str = "target/";
    let target: String = env::args()
        .find(|arg| arg.contains(PREFIX))
        .expect("Unable to find PREFIX");
    let target: String = target
        .chars()
        .skip(PREFIX.len())
        .take(target.find(SUFFIX).expect("Unable to find debug/deps") - PREFIX.len() + SUFFIX.len())
        .collect();

    let target_dir = Utf8PathBuf::from_str(target.as_str()).expect("Can't generate Path from target");
    let dir = target_dir.join(TEMP_FILE);

    let file = match OpenOptions::new().create(true).read(true).write(true).open(&dir) {
        Err(why) => panic!("Couldn't open temporary storage {} : {}", dir, why),
        Ok(file) => file,
    };
    match t {
        LockType::Exclusive => {
            file.lock_exclusive().expect("Can't do exclusive lock");
        }
        LockType::Shared => {
            file.lock_shared().expect("Can't do shared lock");
        }
    };

    file
}
