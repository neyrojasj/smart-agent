// @implements REQ-048 v3 74c0bba3
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let agent_tools_root = Path::new(&manifest_dir).join("../agent-tools");

    println!("cargo:rerun-if-changed=../agent-tools");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(&out_dir).join("skills_registry.rs");
    let mut out = fs::File::create(&out_path).expect("create skills_registry.rs");

    writeln!(out, "/// All files embedded at compile time from agent-tools/.").unwrap();
    writeln!(out, "/// Each entry is (relative_path, content). Path is relative to the").unwrap();
    writeln!(out, "/// agent-tools/ root, e.g. \"skills/weft/SKILL.md\" or \"scripts/afk-claude.sh\".").unwrap();
    writeln!(out, "static EMBEDDED_AGENT_TOOLS: &[(&str, &str)] = &[").unwrap();

    let mut entries: Vec<(String, String)> = Vec::new();
    if agent_tools_root.is_dir() {
        collect_files(&agent_tools_root, &agent_tools_root, &mut entries);
    }
    entries.sort_by(|(a, _), (b, _)| a.cmp(b));

    for (rel_path, content) in &entries {
        println!("cargo:rerun-if-changed=../agent-tools/{rel_path}");
        writeln!(out, "    ({:?}, {:?}),", rel_path, content).unwrap();
    }

    writeln!(out, "];").unwrap();
}

fn collect_files(root: &Path, dir: &Path, out: &mut Vec<(String, String)>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files(root, &path, out);
        } else {
            let rel = path.strip_prefix(root).unwrap_or(&path);
            let rel_str = rel.to_string_lossy().replace('\\', "/");
            let content = fs::read_to_string(&path).unwrap_or_default();
            out.push((rel_str, content));
        }
    }
}
