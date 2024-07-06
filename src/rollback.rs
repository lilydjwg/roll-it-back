use std::path::Path;
use std::io::{ErrorKind, Result as IoResult};
use std::fs::metadata;
use std::os::linux::fs::MetadataExt;

use eyre::Result;
use btrfsutil::subvolume::{Subvolume, SubvolumeIterator};

use crate::cleanup;

pub fn do_rollback(from: &Path, to: &Path, backup: &Path, dry_run: bool) -> Result<()> {
  let need_do_snaphosts = {
    if dry_run {
      let from_parent = backup.parent().unwrap().join(".");
      let backup_parent = backup.parent().unwrap().join(".");
      if is_same_device(&from_parent, &backup_parent)? {
        println!("rename {} to {}", from.display(), backup.display());
        false
      } else {
        true
      }
    } else if let Err(e) = std::fs::rename(from, backup) {
      if e.kind() == ErrorKind::CrossesDevices {
        true
      } else {
        return Err(e.into());
      }
    } else {
      false
    }
  };

  if need_do_snaphosts {
    let sub = Subvolume::get(from)?;
    if dry_run {
      println!("snapshot {} to {}", from.display(), backup.display());
    } else {
      sub.snapshot(backup, None, None)?;
    }
    snapshot_nested_subvolumes(from, backup, None, dry_run)?;
    cleanup::do_cleanup(from, dry_run)?;
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

fn is_same_device(a: &Path, b: &Path) -> IoResult<bool> {
  let stat_a = metadata(a)?;
  let stat_b = metadata(b)?;
  Ok(stat_a.st_dev() == stat_b.st_dev())
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
