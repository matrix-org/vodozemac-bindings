Migrating from libolm
=====================

The Python bindings for vodozemac are structured similarly to the bindings for
libolm, so (other than the exceptions listed below) you can simply import the
classes from the `vodozemac` package rather than from the `olm` package and it
is, for the most part, a drop-in replacement.  For example,

```python
from olm import Account
```

becomes

```python
from vodozemac import Account
```

## Differences

### `account.max_one_time_keys`
The `account.max_one_time_keys` property in libolm returns the maximum number of
one-time keys that the account can store, and only half of that number of
one-time keys should be uploaded to the server at a time.  In vodozemac,
`account.max_one_time_keys` returns the number of one-time keys that should be
uploaded to the server.  It will still mostly work if you only upload
`account.max_one_time_keys / 2` keys as with libolm, but you will have fewer
one-time keys available.

### Pickle formats
Vodozemac uses a different pickling format from libolm.  Vodozemac can read
libolm-formatted pickles using the `*.from_libolm_pickle()` methods.  The
`*.pickle()` and `*.from_pickle()` methods will create and read vodozemac-style
pickles.  If you have existing data, you will need to either migrate the data to
the new format, or support reading both formats.

## Unsupported functionality

Vodozemac does not yet support:
- the `PkEncryption` and `PkDecryption` classes, which are used for key backups;
- the `PkSigning` class, which is used for cross-signing;
- the `ed25519_verify` function;
- the `sha256` function (Python's standard
  [`sha256`](https://docs.python.org/3/library/hashlib.html#hashlib.sha256)
  function can be used instead, but note that libolm's `sha256` function
  base64-encoded, while Python's does not).
