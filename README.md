# Power Letters for Rust

Concise spellings of common Rust operations:

- `C` - `Clone`
- `O` - `ToOwned`
- `S` - `ToString`
- `I` - Ignore `Result`
- `X` - `expect` for `Result` and `Option`

All operations are provided as both functions and methods.
Sometimes one reads better than the other.

In action:

```rust
  let mut parents: Vec<_> = path.ancestors().skip(1).map(|path| {
      if path == Path::new("") {
          let parent_path = pathgen.from_path(path).X();
          let parent_label = S("<root>");
          (parent_path, parent_label)
      } else {
          let parent_path = pathgen.from_path(path).X();
          let parent_label = path.iter().last().X().to_string_lossy().S();
          (parent_path, parent_label)
      }
  }).collect();
```


## Power `Clone`

```
use powerletters::*;

let bagostuff = vec!["a", "b", "c"];
let newbag = bagostuff.C();

// or
let newbag = C(&bagostuff);
```


## Power `ToOwned`

```
use powerletters::*;

let yourpath = Path::new("chill");
let mypath = yourpath.O();

// or
let mypath = O(yourpath);
```


## Power `ToString`

```
use powerletters::*;

let s: String = S("foo");

// or
let s: String = "foo".S();
```


## Power ignore `Result` - kick that `Result` to the curb!

```
use powerletters::*;
use std::io::Write;

let mut buf = Vec::new();
write!(&mut buf, "hello").I();

// or
I(write!(&mut buf, "world"));
```

Note this is superior to `let _ = ...`
because the `let` version is untyped and can
introduce unintended bugs like ignoring futures.


## Power `expect` for `Result` and `Option`.

```
use powerletters::*;

let maybe_thing = Some("thing");
let thing = maybe_thing.X(); // like `.expect("some baloney")`
let good_thing = Ok("thing");
let thing = good_thing.X();

// or
let thing = X(maybe_thing);
let thing = X(good_thing);
```

Panics with helpful messages:

```
let none: Option<i32> = None;
let value = none.X();
// thread 'main' panicked at src/main.rs:3:19:
// impossible `None` option
```

```
let err: Result<i32, _> = Err("something went wrong");
let value = err.X();
// thread 'main' panicked at src/main.rs:3:18:
// impossible `Err` result: something went wrong
```


