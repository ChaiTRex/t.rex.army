use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    let src: PathBuf = std::iter::once("src").collect();
    let dist: PathBuf = std::iter::once("dist").collect();

    WalkDir::new(&src)
        .follow_links(true)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.into_path())
        .filter(|path| path.is_file())
        .map(|path_in_src| {
            let path_in_dist = dist.join(path_in_src.strip_prefix(&src).unwrap());
            (path_in_src, path_in_dist)
        })
        .for_each(|(path_in_src, path_in_dist)| {
            print!("{:?} -> {:?}", path_in_src, path_in_dist);
            let bytes_copied = copy_and_create_dirs(&path_in_src, &path_in_dist).unwrap();
            println!(
                " ({} byte{})",
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
