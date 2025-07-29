use std::fs;
use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir);
    
    // Copie le dossier ressources
    fs::create_dir_all(dest_path.join("ressources")).unwrap();
    fs::copy("./ressources/educations.json", dest_path.join("ressources/educations.json")).unwrap();
    fs::copy("./ressources/personnalities.json", dest_path.join("ressources/personnalities.json")).unwrap();
}