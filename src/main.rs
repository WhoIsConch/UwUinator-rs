use std::fs;
use std::env;
use std::path::Path;
use fs_extra::dir::get_size;

#[derive(Debug)]
struct Settings { 
    path_to_fill: String,
    origin_file: OriginFile,
    amount_of_files: Option<u64>, 
    storage_b: Option<u64>, 
}

impl Settings {
    fn from_args() -> Settings {
        let args: Vec<String> = env::args().collect();

        let mut settings = Settings{ 
            path_to_fill: String::clone(&args[1]), 
            origin_file: OriginFile::default(),
            amount_of_files: None,
            storage_b: None,
        };
    
        for (index, arg) in args.iter().enumerate() {
            match arg.as_str() {
                "--file" => settings.origin_file = OriginFile::from_path(Path::new(&args[index+1])),
                "--amount" => {
                    let amount = args[index+1]
                        .parse::<u64>()
                        .expect("`Amount` argument is not an integer.");
    
                    settings.amount_of_files = Some(amount);
                },
                "--storage" => {
                    let storage_mb = args[index + 1]
                        .parse::<u64>()
                        .expect("`Storage` argument is not an integer.");
    
                    settings.storage_b = Some(storage_mb);
                },
                _ => {}
            }
        }
    
    settings
    }

    fn validate(&self) -> Result<(), &str> {
        if let Some(storage) = self.storage_b {
            if self.origin_file.size > storage {
                return Err("Cannot fill directory with a file bigger than specified size");
            }

            let available_space = get_size(&self.path_to_fill).expect("Invalid path to fill.");

            if storage > available_space {
                return Err("Space to fill exceeds amount of space left in path");
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
struct OriginFile {
    name: String,
    extension: String,
    size: u64,
    full_path: String,
    parent: String,
}

impl OriginFile {
    fn default() -> OriginFile {
        // Gets the default origin file, uwu.png
        let file_path = Path::new("uwu.png");

        OriginFile::from_path(file_path)
    }

    fn from_path(path: &Path) -> OriginFile {
        if !path.exists() {
            panic!("No origin file specified and default file is not found.");
        }
        // Oh my god there has got to be a better way to do this shit
        let file_name = path.file_name()
            .expect("Unable to get file name")
            .to_str()
            .expect("Unable to get file name");

        let file_extension = path.extension()
            .expect("Unable to get file extension")
            .to_str()
            .expect("Unable to get file extension");

        let str_parent = path.parent()
            .expect("Unable to get parent")
            .to_str()
            .expect("Unable to get file path");

        let str_path = path.to_str().expect("could not get path as string i guess");
        let file_size: u64 = get_size(str_path).expect("Unable to get size of origin file.");
        
        OriginFile {
            name: file_name.to_string(),
            extension: file_extension.to_string(),
            size: file_size,
            full_path: str_path.to_string(),
            parent: str_parent.to_string(),
        }
    }
}

fn uwuinate(settings: Settings) {
    let mut counter = 0; // Gets put in file names to avoid duplicates

    loop {
        if let Some(amount) = settings.amount_of_files {
            if counter >= amount {
                return;
            }
        }

        if let Some(storage) = settings.storage_b {
            if settings.origin_file.size * counter >= storage {
                return;
            }
        }

        let mut copy = String::clone(&settings.path_to_fill);
        copy.push_str(&("\\".to_owned() + counter.to_string().as_str() + "." + &settings.origin_file.extension));

        fs::copy(&settings.origin_file.full_path, &copy).expect("Failed to copy data into file");
        counter += 1;
    }
}

fn main() {
    let settings = Settings::from_args();
    let valid = settings.validate();

    if let Err(error) = valid {
        panic!("{error}");
    }

    dbg!(&settings);
    dbg!(&valid);
    uwuinate(settings);
}
