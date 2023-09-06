# ZTF Tarball Reader

This project written in Rust was done with intention of quickly loading ztf alerts and being able to query them.
With this tools someone can know which alert was sent in one night and can also check some details of it.

## Usage

Clone this repository, then execute the binary with a directory to read from. To create a directory
with alerts, download a file from https://ztf.uw.edu/alerts/public/ and unzip it into the `data` folder.
Some alert may have problems loading due to some schema errors.

### Read a directory
```bash
cargo run --release data/ztf_tmp/
```

### Show all alerts after reading a directory
```bash
>> show all
```

### Display data of an alert
```bash
>> ZTF18acsbtx
+--------------+-------------------+-----------+-------------+-----------+-----------+------------+
| ALERT ID     | MJD               | RA        | DEC         | MAGLIM    | MAG       | MAGERR     |
+--------------+-------------------+-----------+-------------+-----------+-----------+------------+
| ZTF18acsbtxd | 58451.25238429988 | 14.474468 | -19.4993322 | 19.534124 | 17.572023 | 0.16631523 |
+--------------+-------------------+-----------+-------------+-----------+-----------+------------+
```

### Close CLI
`CTRL-C` or `CTRL-D`

## Improvement ideas
- Load/store commands history.
- Load/store directories already processed.
- Add help command

## Useful commands when developing
- Run a development version `cargo run data/ztf_tmp/`
- Build a release `cargo build --release`