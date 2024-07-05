```
Usage: roll-it-back [--dry-run] (--from <SUBVOL> --to <SNAPSHOT> --backup <PATH> | --cleanup <SUBVOL>)

Options:
      --from <SUBVOL>     rollback this subvolume...
      --to <SNAPSHOT>     ...to this snapshot...
      --backup <PATH>     ...and save the rolled-back subvolume here
      --cleanup <SUBVOL>  delete this subvolume and all subvolumes nested in it
      --dry-run           show operations to be executed but do not actually execute them
  -h, --help              Print help
  -V, --version           Print version
```
