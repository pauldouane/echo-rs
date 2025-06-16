# echo-rs

**echo-rs** is a Rust reimplementation of the POSIX `echo` utility.  
It prints lines of text passed as arguments, following standard POSIX behavior.

## Features

- Prints arguments separated by spaces, followed by a newline.
- Supports POSIX options:
  - `-n`: do not output the trailing newline.
  - `-e`: enable interpretation of backslash escapes.
  - `-E`: disable interpretation of backslash escapes (default behavior).

## Usage examples

```bash
# Basic output
echo-rs hello world

# Without trailing newline
echo-rs -n "no newline"

# With escape sequences
echo-rs -e "line1\nline2\tTabbed"

# Disable escape sequences (default)
echo-rs -E "line1\nline2"

