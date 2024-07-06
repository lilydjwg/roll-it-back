use std::path::Path;

use eyre::Result;
use btrfsutil::subvolume::{Subvolume, SubvolumeIterator};

pub fn do_rollback(from: &Path, to: &Path, backup: &Path, dry_run: bool) -> Result<()> {
  if dry_run {
    println!("rename {} to {}", from.display(), backup.display());
  } else {
    std::fs::rename(from, backup)?;
  }

  let srctop = if dry_run { from } else { backup };

  let sub = Subvolume::get(to)?;
  if dry_run {
    println!("snapshot {} to {}", to.display(), from.display());
  } else {
    sub.snapshot(from, None, None)?;
  }
  let rebase_src = if dry_run { Some(backup) } else { None };
  snapshot_nested_subvolumes(srctop, from, rebase_src, dry_run)?;

  Ok(())
}

fn snapshot_nested_subvolumes(
  srctop: &Path,
  dsttop: &Path,
  rebase_src: Option<&Path>,
  dry_run: bool,
) -> Result<()> {
  for sub in SubvolumeIterator::new(srctop, None)? {
    let sub = sub?;
    let src = sub.path();
    let mut dst = dsttop.to_owned();
    dst.push(src.strip_prefix(srctop)?);
    if dry_run {
      if dst.is_dir() {
        println!("rmdir {}", dst.display());
      }
      let src = if let Some(rebase) = rebase_src {
        let mut p = rebase.to_owned();
        p.push(src.strip_prefix(srctop)?);
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
