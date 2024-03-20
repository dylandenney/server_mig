use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use walkdir::WalkDir;


#[derive(Debug)]
pub struct OwnedDir {
    pub path: String,
    pub uid: u32,
    pub gid: u32,
}
pub fn gather_owned_directories(root: &Path) -> Vec<OwnedDir> {
    let mut owned_dirs = Vec::new();

   // Change max_depth to get different results for the directory walk
    for entry in WalkDir::new(root).max_depth(4).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if let Ok(metadata) = fs::metadata(path) {
            owned_dirs.push(OwnedDir {
                path: path.to_string_lossy().to_string(),
                uid: metadata.uid(),
                gid: metadata.gid(),
            });
        }
    }

    owned_dirs
}


pub fn generate_ownership_script(owned_dirs: &[OwnedDir]) -> String {
    let mut script = String::from("#!/bin/bash\n");
    for dir in owned_dirs {
        script.push_str(&format!(
            "chown {}:{} {}\n",
            dir.uid, dir.gid, dir.path
        ));
    }
    script
}

