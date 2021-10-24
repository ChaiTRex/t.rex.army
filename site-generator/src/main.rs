use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    let src_path: PathBuf = std::iter::once("src").collect();
    let dist_path: PathBuf = std::iter::once("dist").collect();

    WalkDir::new("src")
        .follow_links(true)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.into_path())
        .filter(|path| path.is_file())
        .map(|src| {
            let dist = dist_path.join(src.strip_prefix(&src_path).unwrap());
            (src, dist)
        })
        .for_each(|(src, dist)| {
            let bytes_copied = copy_and_create_dirs(&src, &dist).unwrap();
            println!(
                "{:?} -> {:?} ({} byte{})",
                src,
                dist,
                bytes_copied,
                if bytes_copied == 1 { "" } else { "s" }
            );
        });
}

fn copy_and_create_dirs<P, Q>(from: P, to: Q) -> std::io::Result<u64>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    match to.as_ref().parent() {
        Some(parent) => std::fs::create_dir_all(parent)?,
        None => (),
    }
    std::fs::copy(from, to)
}
