# Power Letters for Rust

Concise spellings of common Rust operations:

- `C` - `Clone`
- `O` - `ToOwned`
- `S` - `ToString`
- `I` - Ignore `Result`
- `X` - `expect` for `Result` and `Option`

All operations are provided as both functions and methods.
Sometimes one reads better than the other.


### Power `Clone`

```
use powerletters::*:

let bagostuff = vec!["a", "b", "c"];
let newbag = bagostuff.C();

// or
let newbag = C(&bagostuff);
```


### Power `ToOwned`

```
use powerletters::*:

let yourpath = Path::new("chill");
let mypath = yourpath.O();

// or
let mypath = O(yourpath);
```


### Power `ToString`

```
use powerletters::*:

let s: String = S("foo");

// or
let s: String = "foo".S();
```


### Power ignore `Result` - kick that `Result` to the curb!

```
use powerletters::*:

write_logline(&logline).I();

// or
I(write_logline(&logline));
```

Note this is superior to `let _ = ...`
because the `let` version is untyped and can
introduce unintended bugs like ignoring futures.


### Power `expect` for `Result` and `Option`.

```
use powerletters::*:

let maybe_thing = Some("thing");
let thing = maybe_thing.X(); // like `.expect("some baloney")`
let good_thing = Ok("thing");
let thing = good_thing.X();

// or
let thing = X(maybe_thing);
let thing = X(good_thing);
```


