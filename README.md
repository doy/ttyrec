# ttyrec

This crate contains helpers for reading and writing
[ttyrec](https://en.wikipedia.org/wiki/Ttyrec) files.

`Parser` and `Creator` can be used to read and write files manually, and
`Reader` and `Writer` are helpers to provide a nicer API for asynchronous
applications using `tokio`.
