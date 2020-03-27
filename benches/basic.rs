use mindbase::MindBase;
use mindbase_adapter_roamresearch::{Importer, Page};
use std::{fs::File, path::Path};

use criterion::{criterion_group, criterion_main, Criterion};

fn init_mb() -> MindBase {
    let tmpdir = tempfile::tempdir().unwrap();
    let tmpdirpath = tmpdir.path();
    let mb = MindBase::open(&tmpdirpath).unwrap();
    mb
}
fn load_dump_1() -> Vec<Page> {
    let path = Path::new("test_data/dump-1.json");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let pages: Vec<Page> = serde_json::from_reader(&file).unwrap();
    pages
}

fn import(mb: &MindBase, pages: &Vec<Page>) {
    let mut context = Importer::new(&mb).unwrap();
    context.parse_pages(pages).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let mb = init_mb();
    let pages = load_dump_1();

    // c.bench_function("insert_test_dataset", |b| b.iter(|| insert_test_dataset(&mb).unwrap()));
    c.bench_function("import", |b| b.iter(|| import(&mb, &pages)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
