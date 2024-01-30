# txtimg

Text to image & Image to text

## Useage

```
Usage: txtimg[EXE] <COMMAND>

Commands:
  t     Text to image
  m     Image to text
  f     File to image
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Example

```bash
$ ./txtimg t "Hello Rust! 你好世界 🦀" -o output.png
$ file output.png
output.png: PNG image data, 14 x 1, 8-bit/color RGB, non-interlaced
$ ./txtimg m output.png
Hello Rust! 你好世界 🦀
```

## LICENSE

[GNU Affero General Public License v3.0](https://choosealicense.com/licenses/agpl-3.0/)
