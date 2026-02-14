use std::fs;
use std::path::Path;

pub fn rename(old: &Path, new: &Path, dry: bool) -> anyhow::Result<()> {
    if old == new {
        return Ok(());
    }

    let mut target = new.to_path_buf();

    if !dry {
        let mut counter = 1;

        while target.exists() {
            if old == target {
                break;
            }

            let stem = target.file_stem().unwrap_or_default().to_string_lossy();

            let ext = target
                .extension()
                .map(|e| format!(".{}", e.to_string_lossy()))
                .unwrap_or_default();

            let parent = target.parent().unwrap();
            let new_name = format!("{}-{}{}", stem, counter, ext);

            target = parent.join(new_name);
            counter += 1;
        }

        fs::rename(old, &target)?;
    }

    println!("{} -> {}", old.display(), target.display());
    Ok(())
}
