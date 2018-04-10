# 🅱🅰🅼
𝓑𝓐𝓜 turns your PC's motherboard speaker into a fully-fledged monophonic MIDI synthesizer!

## Usage
`bam` takes no arguments. Simply:
```bash
# bam
```

To verify that `bam` is working, use `aconnect -o`; e.g.:
```bash
$ aconnect -o
client 14: 'Midi Through' [type=kernel]
    0 'Midi Through Port-0'
client 128: 'bam' [type=user,pid=16431]
    0 'pcspkr          '
```

### Permissions
To beep, `bam` requires an [`ioctl` `KIOCSOUND` request to `/dev/tty0`](https://www.tldp.org/LDP/lpg/node83.html). On most system configuration, this requires running `bam` as root.

## System Requirements
`bam` works on nix-like systems.

## Build Requirements
`bam` must be compiled with a somewhat recent version of the Rust toolchain. This utility is guaranteed to not compile with versions of `rustc` below 1.15.
