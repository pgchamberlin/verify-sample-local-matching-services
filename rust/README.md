# Notes

## Connection refused on `localhost`

When running `./test_rust.sh` locally on OSX I found I got a "connection refused" error. This turned out to be because Rust wasn't listening on `::1`, so I temporarily removed the following from `/etc/hosts`:

```
::1     locahost
```

Followed by flushing the DNS cache:

```
dscacheutil -flushcache
```
