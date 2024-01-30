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
$ ./txtimg t "Hello Rust! 你好世界 🦀"
$ file output.png
output.png: PNG image data, 14 x 1, 8-bit/color RGB, non-interlaced
$ ./txtimg m output.png
Hello Rust! 你好世界 🦀
```

```bash
$ du -h romeo-and-juliet.txt
144K    romeo-and-juliet.txt
$ md5sum romeo-and-juliet.txt
26ce3b9781d9d75726fab08e3a98e8e7 *romeo-and-juliet.txt
$ ./txtimg f romeo-and-juliet.txt -o raj.png
$ file raj.png
raj.png: PNG image data, 19280 x 1, 8-bit/color RGB, non-interlaced
$ du -h raj.png
60K     raj.png
$ ./txtimg m raj.png > raj.txt
$ md5sum raj.txt
26ce3b9781d9d75726fab08e3a98e8e7 *raj.txt
```

## LICENSE

[GNU Affero General Public License v3.0](https://choosealicense.com/licenses/agpl-3.0/)
