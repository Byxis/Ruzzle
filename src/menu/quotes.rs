use std::fs;
use std::path::Path;

/// Loads quotes from a Markdown file, returning them as a vector of strings.
/// It specifically looks for lines starting with "> " (Markdown blockquotes).
///
/// # Arguments
/// * `path` - The path to the file containing the quotes.
///
/// # Returns
/// A `Vec<String>` where each element is a quote. Returns an empty vector if the file
/// cannot be read or contains no valid quotes.
pub fn load_quotes<P: AsRef<Path>>(path: P) -> Vec<String> {
    fs::read_to_string(path)
        .map(|contents| {
            contents
                .lines()
                .filter_map(|line| {
                    let trimmed = line.trim();
                    if trimmed.starts_with("> ") {
                        // Enlève le "> " du début et retourne la citation
                        Some(trimmed[2..].to_string())
                    } else {
                        // Ignore les autres lignes (titres, dates, lignes vides)
                        None
                    }
                })
                .collect()
        })
        .unwrap_or_else(|_| {
            println!("Could not read quotes file, using default.");
            vec!["Le crabe est le roi de la plage.".to_string()]
        })
}