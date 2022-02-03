use std::io::{Write, Read};
use std::path::Path;
//{fs, };
fn main() {
    const BRAIN_SAVE_URL: &str = "https://github.com/sonicrules1234/aiml_ported/releases/download/brain_save/brain_save.zip";
    const ALICE_BRAIN_URL: &str = "https://github.com/sonicrules1234/aiml_ported/releases/download/alice_brain/alice_brain.zip";
    if !Path::new("alice_brain").exists() {
        let mut alice_brain_zip: Vec<u8> = Vec::new();
        ureq::get(ALICE_BRAIN_URL).call().unwrap().into_reader().read_to_end(&mut alice_brain_zip).unwrap();
        let mut alice_brain = std::fs::File::create("alice_brain.zip").unwrap();
        alice_brain.write_all(alice_brain_zip.as_slice()).unwrap();
        drop(alice_brain);
        let alice_brain = std::fs::File::open("alice_brain.zip").unwrap();
        let mut ab_archive = zip::ZipArchive::new(alice_brain).unwrap();
        ab_archive.extract(".").unwrap();
    }
    if !Path::new("brain_save").exists() {
        let mut brain_save_zip: Vec<u8> = Vec::new();
        ureq::get(BRAIN_SAVE_URL).call().unwrap().into_reader().read_to_end(&mut brain_save_zip).unwrap();
        let mut brain_save = std::fs::File::create("brain_save.zip").unwrap();
        brain_save.write_all(brain_save_zip.as_slice()).unwrap();
        drop(brain_save);
        let brain_save = std::fs::File::open("brain_save.zip").unwrap();
        let mut bs_archive = zip::ZipArchive::new(brain_save).unwrap();
        bs_archive.extract(".").unwrap();
    }
    /*
    for i in 0..ab_archive.len() {
        let mut file = ab_archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
    for i in 0..bs_archive.len() {
        let mut file = bs_archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
    */
}