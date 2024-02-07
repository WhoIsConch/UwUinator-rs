use std::{fs, fmt::Write, env, path::Path};
use fs2::free_space;
use fs_extra::dir::get_size;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use nanoid::nanoid;

struct Settings { 
    path_to_fill: String,
    origin_file: OriginFile,
    space_in_path: u64,
    amount_of_files: Option<u64>, 
    storage_b: Option<u64>, 
}

impl Settings {
    fn from_args() -> Settings {
        let args: Vec<String> = env::args().collect();
        let path_to_fill = String::clone(&args[1]);

        let mut settings = Settings{ 
            path_to_fill: String::clone(&path_to_fill), 
            origin_file: OriginFile::default(),
            space_in_path: free_space(&path_to_fill).expect("Invalid path to fill."),
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

            if storage > self.space_in_path {
                return Err("Space to fill exceeds amount of space left in path");
            }
        }

        if let Some(file_amount) = self.amount_of_files {
            if (file_amount * self.origin_file.size) > self.space_in_path {
                return Err("Directory cannot fit that many files in it.");
            }
        }

        let path_exists = Path::new(&self.path_to_fill).exists();

        if !path_exists {
            return Err("Directory does not exist.");
        }

        let file_exists = Path::new(&self.origin_file.full_path).exists();

        if !file_exists {
            return Err("Origin File does not exist");
        }

        Ok(())
    }
}

struct OriginFile {
    extension: String,
    size: u64,
    full_path: String,
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
        let file_extension = path.extension()
            .expect("Unable to get file extension")
            .to_str()
            .expect("Unable to get file extension");

        let str_path = path.to_str().expect("could not get path as string i guess");
        let file_size: u64 = get_size(str_path).expect("Unable to get size of origin file.");
        
        OriginFile {
            extension: file_extension.to_string(),
            size: file_size,
            full_path: str_path.to_string(),
        }
    }
}

enum ProgressBarMode {
    File,
    Space,
}

fn uwuinate(settings: Settings) {
    // I can almost guarantee not all of these variables need to be mutable.
    let mut counter = 0; // Gets put in file names to avoid duplicates
    let mut total_filled: u64 = 0;
    let mut template = "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})";
    let mut progress_bar_mode = ProgressBarMode::Space;

    // Decide which progress bar template and total_filled metric to use
    let pb_space = match (settings.storage_b, settings.amount_of_files) {
        (Some(storage_b), _) => storage_b,
        (None, Some(amount_of_files)) => {
            template = "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})";
            progress_bar_mode = ProgressBarMode::File;
            amount_of_files
        },
        (None, None) => settings.space_in_path,
    };
    
    println!("Welcome to the UwUinator v2");
    println!("-- Created by WhoIsConch --");
    println!("Now UwUinating path: {0}", settings.path_to_fill);
    println!("With file: {0}", settings.origin_file.full_path);

    // Construct the progress bar
    let progress_bar = ProgressBar::new(pb_space);

    progress_bar.set_style(ProgressStyle::with_template(template)
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    loop {
        if let Some(amount) = settings.amount_of_files {
            if counter >= amount {
                break;
            }
        }

        if let Some(storage) = settings.storage_b {
            if total_filled >= storage {
                break;
            }
        }

        let destination = format!("{0}\\{1}.{2}", &settings.path_to_fill, nanoid!(10), &settings.origin_file.extension.as_str());

        // Copy the file
        let status = fs::copy(&settings.origin_file.full_path, &destination);

        if let Err(_) = status {
            break;
        }

        total_filled += settings.origin_file.size;
        counter += 1;

        // Update the progress bar 
        progress_bar.set_position(match progress_bar_mode {
            ProgressBarMode::File => counter,
            ProgressBarMode::Space => total_filled,
        });
    }

    progress_bar.finish();
    println!("Successfully UwUinated path in {:#?}", progress_bar.elapsed())
}

fn main() {
    // Get and validate settings
    let settings = Settings::from_args();
    let valid = settings.validate();

    // Just panic if there's an error for now
    // This should change later
    if let Err(error) = valid {
        panic!("{error}");
    }

    uwuinate(settings);
}
