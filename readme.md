# Joule

This CLI tool was made for monitoring the energy consumption, by taking energy snapshots of the CPU at a given point in time.

## Howto use it

```bash
# clone the repository
git clone https://github.com/jdrouet/joule
# go into the repository folder
cd joule
# build it
cargo build --release
# use it
sudo ./target/release/joule snapshot path/to/first-snapshot.json
sudo ./target/release/joule snapshot path/to/second-snapshot.json
./target/release/joule compare path/to/first-snapshot.json path/to/second-snapshot.json
```

You need to be a super user to execute the `snapshot` command, to access the powercap files.

## More information

⚠️ This package has nothing related to any French person ⚠️ 
