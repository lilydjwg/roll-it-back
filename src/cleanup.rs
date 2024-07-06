use std::path::Path;
use std::iter::once;

use btrfsutil::subvolume::{Subvolume, SubvolumeIterator, SubvolumeIteratorFlags};
use eyre::Result;

pub fn do_cleanup(subvol: &Path, dry_run: bool) -> Result<()> {
  for sub in SubvolumeIterator::new(subvol, SubvolumeIteratorFlags::POST_ORDER)?
    .chain(once(Subvolume::get(subvol)))
  {
    let sub = sub?;
    if dry_run {
      println!("delete subvolume {}", sub.path().display());
    } else {
      sub.delete(None)?;
    }
  }

  Ok(())
}
