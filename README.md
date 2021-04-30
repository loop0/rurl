# rurl
rurl is a curl-like cli tool made in rust, the difference is that it takes its
params from a json file so you can have all different requests saved and rerun
from there.

This tools if far from being feature complete. The verbose mode outputs the
response headers to the stderr so you can still redirect the output to a file.

## Usage

```sh
$ rurl examples/whatismyip.json
{"ip":"192.168.0.1"}
```

## TODO
- [] body from file
- [] status code
- [] better error handling


## Motivation
I'm using this project to learn rust :)