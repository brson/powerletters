# Power Letters for Rust

Concise and visually distinct spellings of common Rust operations.


### "to `String`"

```
let s: String = S("foo");

// or
let s: String = "foo".S();
```


### "Clone"

```
let bagostuff = vec!["a", "b", "c"];
let newbag = bagostuff.C();

// or
let newbag = C(bagostuff);
```



### "Expect Ok or Some"

```
let maybe_thing = Some("thing");
let thing = maybe_thing.X(); // like `.expect("some baloney")`
let good_thing = Ok("thing");
let thing = good_thing.X();

// or
let thing = X(maybe_thing);
let thing = X(good_thing);
```


### "To Owned"

```
let yourpath = &Path("chill");
let mypath = yourpath.O();

// or
let mypath = O(yourpath);
```


### "Ignore Result"

```
write_logline(&logline).I();

// or
I(write_logline(&logline));
```

Note this is superiour to `let _ = ...`
because the `let` version is untyped and can
introduce unintented bugs like ignoring futures.

