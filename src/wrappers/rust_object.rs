use ron::de::{self, from_reader};
use serde::{de::DeserializeOwned, Deserialize};
use std::fs::File;

/// Ron Data Generic Marker
///
/// * Allows the user to decode a ron file whilst return the actual data
///
///

/// First RonData implementation
///
/// Example:
///
/// ```
/// use serde::{de::DeserializeOwned, Deserialize};
///
/// #[derive(Debug, Deserialize)]
/// pub struct ConfigDisplay {
///     pub title: String,
///     pub dimensions: Option<(u32, u32)>,
/// }
///
/// let config: ConfigDisplay = RonData::new("display.ron");
/// println!("{:?}", &config.title);
/// println!("{:?}", &config.dimensions);
/// ````
///
pub struct RonData<T>(T);

impl<T: DeserializeOwned> RonData<T> {
    /// * Parameter -> file: &'static str
    /// Takes in path of file
    ///
    /// looks at ``` {CARGO_MANIFEST_DIR}/assets/config/{file}```
    ///
    /// > It's automatically assumed that your using an assets/config,
    /// > You could change this to any path you want, I just made it more convenient for my use.
    ///
    pub fn new(file: &'static str) -> T {
        let input_path = format!("{}/assets/config/{}", env!("CARGO_MANIFEST_DIR"), file);
        let file = File::open(input_path).expect("Failed opening RON file");

        let config: T = match from_reader(file) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load config: {}", e);

                std::process::exit(1);
            }
        };

        config
    }
}
