use std::path::Path;
use anyhow::{Result, Context};
use walkdir::WalkDir;

pub async fn copy_dir_recursive<P: AsRef<Path>>(src: P, dst: P) -> Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();
    
    if !src.exists() {
        return Err(anyhow::anyhow!("Source directory does not exist: {}", src.display()));
    }

    // Create destination directory
    tokio::fs::create_dir_all(dst).await
        .context("Failed to create destination directory")?;

    // Copy all files and directories recursively
    for entry in WalkDir::new(src) {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();
        
        // Skip the source directory itself
        if path == src {
            continue;
        }

        // Calculate relative path from source
        let relative_path = path.strip_prefix(src)
            .context("Failed to strip source prefix")?;
        
        let target_path = dst.join(relative_path);

        if entry.file_type().is_dir() {
            // Create directory
            tokio::fs::create_dir_all(&target_path).await
                .with_context(|| format!("Failed to create directory: {}", target_path.display()))?;
        } else {
            // Create parent directory if it doesn't exist
            if let Some(parent) = target_path.parent() {
                tokio::fs::create_dir_all(parent).await
                    .with_context(|| format!("Failed to create parent directory: {}", parent.display()))?;
            }

            // Copy file
            tokio::fs::copy(&path, &target_path).await
                .with_context(|| format!("Failed to copy file: {} -> {}", path.display(), target_path.display()))?;
        }
    }

    Ok(())
}

pub async fn ensure_directory_exists<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    
    if !path.exists() {
        tokio::fs::create_dir_all(path).await
            .with_context(|| format!("Failed to create directory: {}", path.display()))?;
    }

    Ok(())
}

pub async fn read_file_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    let path = path.as_ref();
    
    tokio::fs::read_to_string(path).await
        .with_context(|| format!("Failed to read file: {}", path.display()))
}

pub async fn write_string_to_file<P: AsRef<Path>>(path: P, content: &str) -> Result<()> {
    let path = path.as_ref();
    
    // Create parent directory if it doesn't exist
    if let Some(parent) = path.parent() {
        ensure_directory_exists(parent).await?;
    }

    tokio::fs::write(path, content).await
        .with_context(|| format!("Failed to write file: {}", path.display()))
}

pub async fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    tokio::fs::metadata(path.as_ref()).await.is_ok()
}

pub async fn is_directory<P: AsRef<Path>>(path: P) -> bool {
    match tokio::fs::metadata(path.as_ref()).await {
        Ok(metadata) => metadata.is_dir(),
        Err(_) => false,
    }
}

pub async fn list_directories<P: AsRef<Path>>(path: P) -> Result<Vec<std::path::PathBuf>> {
    let path = path.as_ref();
    let mut directories = Vec::new();
    
    let mut entries = tokio::fs::read_dir(path).await
        .with_context(|| format!("Failed to read directory: {}", path.display()))?;

    while let Some(entry) = entries.next_entry().await? {
        if entry.file_type().await?.is_dir() {
            directories.push(entry.path());
        }
    }

    directories.sort();
    Ok(directories)
}

pub async fn find_files_with_extension<P: AsRef<Path>>(
    path: P, 
    extension: &str
) -> Result<Vec<std::path::PathBuf>> {
    let path = path.as_ref();
    let mut files = Vec::new();
    
    for entry in WalkDir::new(path) {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();
        
        if entry.file_type().is_file() {
            if let Some(ext) = path.extension() {
                if ext == extension {
                    files.push(path.to_path_buf());
                }
            }
        }
    }

    files.sort();
    Ok(files)
}

pub async fn backup_file<P: AsRef<Path>>(path: P) -> Result<std::path::PathBuf> {
    let path = path.as_ref();
    
    if !path.exists() {
        return Err(anyhow::anyhow!("File does not exist: {}", path.display()));
    }

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let backup_path = if let Some(parent) = path.parent() {
        let file_stem = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("backup");
        let extension = path.extension()
            .and_then(|s| s.to_str())
            .map(|s| format!(".{}", s))
            .unwrap_or_default();
        
        parent.join(format!("{}.backup.{}{}", file_stem, timestamp, extension))
    } else {
        std::path::PathBuf::from(format!("{}.backup.{}", path.display(), timestamp))
    };

    tokio::fs::copy(path, &backup_path).await
        .with_context(|| format!("Failed to create backup: {} -> {}", path.display(), backup_path.display()))?;

    Ok(backup_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_copy_dir_recursive() {
        let temp_dir = tempdir().unwrap();
        let src_dir = temp_dir.path().join("src");
        let dst_dir = temp_dir.path().join("dst");

        // Create source structure
        tokio::fs::create_dir_all(src_dir.join("subdir")).await.unwrap();
        tokio::fs::write(src_dir.join("file1.txt"), "content1").await.unwrap();
        tokio::fs::write(src_dir.join("subdir/file2.txt"), "content2").await.unwrap();

        // Copy directory
        copy_dir_recursive(&src_dir, &dst_dir).await.unwrap();

        // Verify copied structure
        assert!(dst_dir.join("file1.txt").exists());
        assert!(dst_dir.join("subdir/file2.txt").exists());
        
        let content1 = tokio::fs::read_to_string(dst_dir.join("file1.txt")).await.unwrap();
        assert_eq!(content1, "content1");
    }

    #[tokio::test]
    async fn test_ensure_directory_exists() {
        let temp_dir = tempdir().unwrap();
        let new_dir = temp_dir.path().join("new/nested/dir");

        ensure_directory_exists(&new_dir).await.unwrap();
        assert!(new_dir.exists());
        assert!(new_dir.is_dir());
    }

    #[tokio::test]
    async fn test_file_operations() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let content = "Hello, World!";

        // Write file
        write_string_to_file(&file_path, content).await.unwrap();
        assert!(file_exists(&file_path).await);

        // Read file
        let read_content = read_file_to_string(&file_path).await.unwrap();
        assert_eq!(read_content, content);

        // Backup file
        let backup_path = backup_file(&file_path).await.unwrap();
        assert!(backup_path.exists());
        
        let backup_content = read_file_to_string(&backup_path).await.unwrap();
        assert_eq!(backup_content, content);
    }
}
