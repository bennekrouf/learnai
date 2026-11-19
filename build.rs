//! Build script.
//!
//! * Embeds the course: every file under `content/` becomes a `(path, text)`
//!   pair in `$OUT_DIR/content_files.rs`, so the app carries its lessons and
//!   needs no folder beside it. Editing a lesson rebuilds the app.
//! * Embeds the Windows app icon into the .exe resource, when
//!   `assets/icon.ico` is there. A no-op on other targets.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=content");
    println!("cargo:rerun-if-changed=assets/icon.ico");

    embed_content();

    #[cfg(target_os = "windows")]
    windows_icon();
}

fn embed_content() {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into()))
        .join("content");
    let mut files = Vec::new();
    collect(&root, &mut files);
    files.sort();

    let mut out = String::from("pub static FILES: &[(&str, &str)] = &[\n");
    for path in &files {
        println!("cargo:rerun-if-changed={}", path.display());
        let rel = path
            .strip_prefix(&root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");
        let _ = writeln!(
            out,
            "    ({rel:?}, include_str!({:?})),",
            path.display().to_string()
        );
    }
    out.push_str("];\n");

    let dest = PathBuf::from(std::env::var("OUT_DIR").unwrap_or_else(|_| ".".into()))
        .join("content_files.rs");
    if let Err(e) = std::fs::write(&dest, out) {
        println!("cargo:warning=could not write {}: {e}", dest.display());
    }
}

/// The course's Markdown and YAML files, in every folder under `dir`.
fn collect(dir: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            println!("cargo:rerun-if-changed={}", path.display());
            collect(&path, files);
        } else if matches!(
            path.extension().and_then(|e| e.to_str()),
            Some("md" | "yaml")
        ) {
            files.push(path);
        }
    }
}

#[cfg(target_os = "windows")]
fn windows_icon() {
    let icon_path = std::path::Path::new("assets/icon.ico");
    if !icon_path.exists() {
        println!("cargo:warning=assets/icon.ico not found — the .exe will ship without an embedded icon.");
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("assets/icon.ico");
    res.set("FileDescription", "LearnAI");
    res.set("ProductName", "LearnAI");
    res.set("CompanyName", "Mayorana");
    res.set("LegalCopyright", "© 2026 Mayorana");
    if let Err(e) = res.compile() {
        println!("cargo:warning=Failed to embed Windows icon resource: {e}");
    }
}
