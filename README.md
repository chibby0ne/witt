## witt

i.e: what is the time

Convert ***dirty*** timestamps to dates in UTC and Local Timezones


### Usage

```
Usage: witt [TIMES]...

Arguments:
  [TIMES]...  Time as timestamp in utc

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Passing some timestamps:

```
$ witt 1739736696 1730000000
Item:               0                             1
Read:               1739736696                    1730000000
As timestamp:       1739736696                    1730000000
In UTC:             2025-02-16 20:11:36 UTC       2024-10-27 03:33:20 UTC
In Local Timezone:  2025-02-16 21:11:36 +01:00    2024-10-27 04:33:20 +01:00
```

It can also read malformed timestamps and tries to interpret it on a best case basis:

```
witt -- 12381213_312 -81231231
Item:               0                             1
Read:               12*381-213_312                -81231231
As timestamp:       12381213312                   81231231
In UTC:             2362-05-07 01:55:12 UTC       1972-07-29 04:13:51 UTC
In Local Timezone:  2362-05-07 03:55:12 +02:00    1972-07-29 05:13:51 +01:00
```


### Is it useful?

Yes

In all seriousness, helps to see dates of `flake.lock` `lastModified` field (`
bat flake.lock | jq '.nodes.nixpkgs.locked.lastModified'`) since I cannot read
simple read timestamps and know which day it was (not yet at least).
