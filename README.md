# Regdump Parser

Parse Linux debugfs regmap register dumps and pretty print them.
Requires generating register maps with [something like this C macro parser I wrote](https://github.com/calebccff/WheresThatDefine).

## Usage

smb2regdump parses files like those from the kernels regmap debugfs dump,
in the format `addr: val`, something like:

```sh
1304: 02
1305: 63
1306: 00
```


```sh
Usage: smb2regdump [OPTIONS] --regdump <REGDUMP>

Options:
  -r, --regdump <REGDUMP>        
  -s, --start-addr <START_ADDR>  
  -l, --length <LENGTH>          
  -h, --help                     Print help
  -V, --version                  Print version
```

You can invoke it directly and even watch, if you don't mind dumping the whole
address space.... Does the kernel have a better way of doing this?

```sh
smb2regdump -r /sys/kernel/debug/regmap/0-02/registers -s 0x1000 -l 0xff
```
