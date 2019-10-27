# ttyrec

This crate contains helpers for reading and writing
[ttyrec](https://en.wikipedia.org/wiki/Ttyrec) files.

`Parser` and `Creator` can be used to read and write files manually, and
`Reader` and `Writer` are helpers to provide a nicer API for asynchronous
applications using `tokio`.

If you are not using `tokio`, the `tokio` dependencies can be removed by
building with `default_features = false` (by default, the `"async"`
feature is enabled which provides `tokio` support).
