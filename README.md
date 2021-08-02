# Overview
`pfin` is a program that waits for a process to exit and terminates. It currently only works on Linux and requires a kernel version >=5.3, because it relies on a `pidfd_open` syscall.

`pfin` can wait for any process, child or not, doesn't require root privileges and can wait for processes owned by other users, including root.

# Usage
```pfin [-h] [-v] PID```

# Building
To build `pfin`, you need:
- Rust toolchain. If you don't have it, follow the instructions [here](https://www.rust-lang.org/tools/install)
- `make`

```
git clone https://github.com/tedwa/pfin
cd pfin
make
```

# Contributing
This tool was initially planned to work on other Unix-like operating systems, not just Linux. If you know a technique that can achieve similar results to `pidfd_open` and works on other OS/older Linux kernel version, please open an issue or submit a pull request.

Keep in mind that your suggested technique has to fulfill these requirements:
- A process to wait for doesn't have to be a child
- Works on processes owned by other users, including root
- Does not require root privileges

# TODO
- Manual page
