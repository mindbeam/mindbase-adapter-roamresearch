use std::{fs::File, path::Path};

use mindbase::MindBase;
use mindbase_adapter_roamresearch::{file_format::Page, importer::Importer};

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("roam-test-dump-1.json");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let pages: Vec<Page> = serde_json::from_reader(&file)?;

    let dir = std::env::current_dir().unwrap();
    println!("Loading database in {}", dir.as_path().display());
    let mb = MindBase::open(&dir.as_path()).unwrap();

    // TODO 1 - reconstitute this from previous runs

    let mut context = Importer::new(&mb)?;

    context.parse_pages(pages)?;

    Ok(())
}
