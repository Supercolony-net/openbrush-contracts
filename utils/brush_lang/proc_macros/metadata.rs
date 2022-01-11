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
    convert,
    env,
    fmt,
    fs::{
        File,
        OpenOptions,
    },
    io,
    io::{
        BufReader,
        Seek,
        SeekFrom,
    },
    path::PathBuf,
    process,
    str::FromStr,
    string,
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
/// It stores file in the nearest target folder
/// from the directory where the build command has been invoked(output of `pwd` command).
/// If the directory doesn't contain `Cargo.toml` file,
/// it will try to find `Cargo.toml` in the upper directories.
pub(crate) fn get_locked_file(t: LockType) -> File {
    let manifest_path = locate_manifest().unwrap_or_else(|error| panic!("Unable to locate manifest: {:?}", error));

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

/// Returns the Cargo manifest path of the surrounding crate.
///
/// The path is retrieved by parsing the output of `cargo locate-project`.
pub fn locate_manifest() -> Result<PathBuf, LocateManifestError> {
    let cargo = env::var("CARGO").unwrap_or("cargo".to_owned());
    let output = process::Command::new(cargo).arg("locate-project").output()?;
    if !output.status.success() {
        return Err(LocateManifestError::CargoExecution { stderr: output.stderr })
    }

    let output = String::from_utf8(output.stdout)?;
    let parsed = json::parse(&output)?;
    let root = parsed["root"].as_str().ok_or(LocateManifestError::NoRoot)?;
    Ok(PathBuf::from(root))
}

/// Errors that can occur while retrieving the cargo manifest path.
#[derive(Debug)]
pub enum LocateManifestError {
    /// An I/O error that occurred while trying to execute `cargo locate-project`.
    Io(io::Error),
    /// The command `cargo locate-project` did not exit successfully.
    CargoExecution {
        /// The standard error output of `cargo locate-project`.
        stderr: Vec<u8>,
    },
    /// The output of `cargo locate-project` was not valid UTF-8.
    StringConversion(string::FromUtf8Error),
    /// An error occurred while parsing the output of `cargo locate-project` as JSON.
    ParseJson(json::Error),
    /// The JSON output of `cargo locate-project` did not contain the expected "root" string.
    NoRoot,
}

impl fmt::Display for LocateManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocateManifestError::Io(err) => {
                write!(
                    f,
                    "An I/O error occurred while trying to execute `cargo locate-project`: {}",
                    err
                )
            }
            LocateManifestError::CargoExecution { stderr } => {
                write!(
                    f,
                    "The command `cargo locate-project` did not exit successfully.\n\
                Stderr: {}",
                    String::from_utf8_lossy(stderr)
                )
            }
            LocateManifestError::StringConversion(err) => {
                write!(f, "The output of `cargo locate-project` was not valid UTF-8: {}", err)
            }
            LocateManifestError::ParseJson(err) => {
                write!(f, "The output of `cargo locate-project` was not valid JSON: {}", err)
            }
            LocateManifestError::NoRoot => {
                write!(
                    f,
                    "The JSON output of `cargo locate-project` did not contain the expected \"root\" string."
                )
            }
        }
    }
}

impl std::error::Error for LocateManifestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LocateManifestError::Io(err) => Some(err),
            LocateManifestError::CargoExecution { stderr: _ } => None,
            LocateManifestError::StringConversion(err) => Some(err),
            LocateManifestError::ParseJson(err) => Some(err),
            LocateManifestError::NoRoot => None,
        }
    }
}

impl convert::From<io::Error> for LocateManifestError {
    fn from(source: io::Error) -> Self {
        LocateManifestError::Io(source)
    }
}

impl convert::From<string::FromUtf8Error> for LocateManifestError {
    fn from(source: string::FromUtf8Error) -> Self {
        LocateManifestError::StringConversion(source)
    }
}

impl convert::From<json::Error> for LocateManifestError {
    fn from(source: json::Error) -> Self {
        LocateManifestError::ParseJson(source)
    }
}
