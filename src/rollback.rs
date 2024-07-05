use std::path::{Path, PathBuf};
use std::iter::once;

use eyre::Result;
use btrfsutil::subvolume::{Subvolume, SubvolumeIterator};

pub fn do_rollback(from: &str, to: &str, backup: &str, dry_run: bool) -> Result<()> {
  if dry_run {
    println!("rename {} to {}", from, backup);
  } else {
    std::fs::rename(from, backup)?;
  }

  let snap: &Path = to.as_ref();
  let top: &Path = if dry_run {
    from.as_ref()
  } else {
    backup.as_ref()
  };
  let dsttop: &Path = from.as_ref();

  for (idx, sub) in once(Subvolume::get(snap)).chain(SubvolumeIterator::new(top, None)?).enumerate() {
    let sub = sub?;
    let src = sub.path();
    let mut dst = dsttop.to_owned();
    if idx == 0 {
      dst.push(src.strip_prefix(snap)?);
    } else {
      dst.push(src.strip_prefix(top)?);
    }
    if dry_run {
      if idx != 0 && dst.is_dir() {
        println!("rmdir {}", dst.display());
      }
      let src = if idx != 0 {
        let mut p = PathBuf::new();
        p.push(backup);
        p.push(src.strip_prefix(top)?);
        p
      } else {
        src.to_owned()
      };
      println!("snapshot {} to {}", src.display(), dst.display());
    } else {
      let _ = std::fs::remove_dir(&dst);
      sub.snapshot(&*dst, None, None)?;
    }
  }

  Ok(())
}

