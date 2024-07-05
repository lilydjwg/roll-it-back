use std::path::Path;
use std::iter::once;

use btrfsutil::subvolume::{Subvolume, SubvolumeIterator, SubvolumeIteratorFlags};
use eyre::Result;

pub fn do_cleanup(subvol: &str, dry_run: bool) -> Result<()> {
  for sub in SubvolumeIterator::new(
    AsRef::<Path>::as_ref(subvol), SubvolumeIteratorFlags::POST_ORDER,
  )?.chain(once(Subvolume::get(AsRef::<Path>::as_ref(subvol))))
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
