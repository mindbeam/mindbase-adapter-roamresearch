use std::path::PathBuf;
use structopt::StructOpt;

/// Commandline tool for importing and exporting RoamResearch files for MindBase
#[derive(StructOpt, Debug)]
#[structopt(name = "cli")]
struct Opt {
    /// Path to your MindBase
    #[structopt(short, long, parse(from_os_str))]
    mindbase: PathBuf,

    /// Import RoamResearch file into MindBase
    #[structopt(short, long, parse(from_os_str))]
    import: Option<PathBuf>,

    /// Export Mindbase contents into RoamResearch file
    #[structopt(short, long, parse(from_os_str))]
    export: Option<PathBuf>,
}
use std::fs::File;

use mindbase::MindBase;
use mindbase_adapter_roamresearch::{file_format::Page, importer::Importer};

fn main() -> Result<(), std::io::Error> {
    let opt = Opt::from_args();

    let mb = MindBase::open(&opt.mindbase.as_path()).unwrap();

    if let Some(file) = opt.import {
        let path = file.as_path();
        let display = path.display();

        let file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        println!("Importing {}", display);
        let pages: Vec<Page> = serde_json::from_reader(&file)?;
        let mut context = Importer::new(&mb)?;

        context.parse_pages(&pages)?;
        println!("Done")
    } else if let Some(_file) = opt.export {
        unimplemented!()
        //TODO
    }

    Ok(())
}
