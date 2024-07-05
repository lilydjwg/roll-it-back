use clap::{command, arg, crate_name, ArgGroup};

mod rollback;
mod cleanup;

fn main() {
  let mut cmd = command!()
    .override_usage(concat!(crate_name!(), " [--dry-run] (--from <SUBVOL> --to <SNAPSHOT> --backup <PATH> | --cleanup <SUBVOL>)"))
    .arg(arg!(--from <SUBVOL> "rollback this subvolume..."))
    .arg(arg!(--to <SNAPSHOT> "...to this snapshot..."))
    .arg(arg!(--backup <PATH> "...and save the rolled-back subvolume here"))
    .arg(arg!(--cleanup <SUBVOL> "delete this subvolume and all subvolumes nested in it")
      .conflicts_with("rollback"))
    .arg(arg!(--"dry-run" "show operations to be executed but do not actually execute them"))
    .group(ArgGroup::new("rollback")
      .args(["from", "to", "backup"])
      .multiple(true)
      .requires_all(["from", "to", "backup"]));

  let args = cmd.get_matches_mut();
  let dry_run = args.get_flag("dry-run");
  if let Some(from) = args.get_one::<String>("from") {
    let to = args.get_one::<String>("to").unwrap();
    let backup = args.get_one::<String>("backup").unwrap();
    rollback::do_rollback(from, to, backup, dry_run).unwrap();
  } else if let Some(cleanup) = args.get_one::<String>("cleanup") {
    cleanup::do_cleanup(cleanup, dry_run).unwrap();
  } else {
    let _ = cmd.print_help();
    std::process::exit(1);
  }
}
