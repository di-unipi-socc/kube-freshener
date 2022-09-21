pub enum CMD {
    ListKnownImages,
    ListManifestsIgnore,
    AddKnownImage,
    AddManifestIgnore,
    DeleteKnownImage,
    DeleteManifestIgnore,
    NotExistingCommand  
}

impl CMD {

    fn get_types() -> Vec<String> {
        vec!["sidecar", "mr"]
        .into_iter()
        .map(|k| k.to_owned())
        .collect()
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "list-known-images" => Self::ListKnownImages,
            "list-manifest-ignore" => Self::ListManifestsIgnore,
            "add-known-image" => Self::AddKnownImage,
            "add-manifest-ignore" => Self::AddManifestIgnore,
            "delete-known-image" => Self::DeleteKnownImage,
            "delete-manifest-ignore" => Self::DeleteManifestIgnore,
            _ => Self::NotExistingCommand
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::ListKnownImages => "list-known-images",
            Self::ListManifestsIgnore => "list-manifest-ignore",
            Self::AddKnownImage => "add-known-image",
            Self::AddManifestIgnore => "add-manifest-ignore",
            Self::DeleteKnownImage => "delete-known-image",
            Self::DeleteManifestIgnore => "delete-manifest-ignore",
            _ => ""
        }
    }

    pub fn check_args(&self, args: &Vec<String>) -> bool {
        match self {
            Self::AddKnownImage => {
                if args.len() < 5 {
                    println!("[X] You're missing parameters: [cargo run {} <name> <image> <kind>]",
                        Self::AddKnownImage.to_str()
                    );
                    return false;
                }

                if !Self::get_types().into_iter().any(|k| k == args[4]) {
                    println!("[X] <kind> parameter must be: {:?}", Self::get_types());
                    return false;
                }

                true
            },
            Self::AddManifestIgnore => {
                if args.len() < 3 {
                    println!("[X] You're missing parameters: [cargo run {} <filename>]",
                        Self::AddManifestIgnore.to_str(),
                    );
                    return false;
                }
                true
            },
            Self::DeleteKnownImage => {
                if args.len() < 3 {
                        println!("[X] You're missing <name> parameter: [cargo run {} <name>]",
                        Self::DeleteKnownImage.to_str()
                    );
                    return false;
                }
                true
            },
            Self::DeleteManifestIgnore => {
                if args.len() < 3 {
                        println!("[X] You're missing <name> parameter: [cargo run {} <name>]",
                        Self::DeleteKnownImage.to_str()
                    );
                    return false;
                }
                true
            }
            _ => true
        }
    }
}