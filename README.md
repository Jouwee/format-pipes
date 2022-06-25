Collection of quality-of-life formatting pipes intended for using in a *NIX terminal.

# QRCode

Generates a QRCode in the terminal for easy mobile data transfer

```
# Generates QRCode from stdin
echo hello my name is jeff | qrcode

# Generates QRCode from clipboard (Text only)
qrcode -c
```

# Building

```
cargo build
```

I recommend running `setup.sh` afterwards for setting up convinence aliases