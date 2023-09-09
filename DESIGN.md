# `process-image`
## Basic Idea
A rust crate that helps accessing values from a "process image" (PI).  A "process image" is a block of data in the industrial automation world.  Usually, you can access its contents in two ways:

1. Absolute access:  By directly addressing values in the PI, you can read or write data.  The relevant standard (IEC 61131-3) defines the syntax for this.  Examples: `%IX15.3`, `%QW7`
2. Symbolic access:  "Tags" are assigned to addresses to reference them.  The code then uses these tags like variables to interact with the PI.

`process-image` should be a zero cost abstraction for creating such tags over a PI.

### Requirements
- Must be zero-cost.
- Must be able to use externally-owned slices (mutable or immutable) for the PI.

## Ideas
### 1. Smart data structure
```rust
struct Pi<'a> {
    pub btn_start: bool,
    pub btn_stop: bool,
    pub speed: u16,
    _pi_ref: &mut 'a [u8],
}

impl<'a> Pi<'a> {
    #[inline(always)]
    pub fn new(pi: &mut 'a [u8]) -> Self {
        Self {
            btn_start: pi[/* magic access to certain bit */],
            btn_stop: pi[/* magic access to certain bit */],
            speed: pi[/* magic access to certain byte */]
            _pi_ref: pi,
        }
    }
}

impl<'a> Drop for Pi<'a> {
    #[inline(always)]
    fn drop(&mut self) {
        pi[/* magic access to certain bit */] = self.btn_start;
        pi[/* magic access to certain bit */] = self.btn_stop;
        pi[/* magic access to certain bytes */] = self.speed;
    }
}
```

```rust
let mut pi = Pi::new(&mut buf);

pi.btn_start = true;
pi.speed -= 10;

/* drop updates values here */

write_pi(&buf);
```

#### Advantages
- Tag access is literally variable/field access.  Makes the code look most natural.
- Probably can be made generic about mutability by storing the `_pi_ref` in an `Option<>`.

#### Disadvantages
- Unclear if this will always be zero-cost.  If it does not get optimized, it blows up to a very large size.
- Sharing a `&mut Pi` to a different function may immediately kill the zero-costness.

### 2. Opaque struct with acessors
```rust
struct PiMut<'a> {
    pi: &mut 'a [u8],
}

impl<'a> PiMut<'a> {
    pub fn new() { /* ... */ }

    #[inline(always)]
    pub fn btn_start(&mut self) -> &mut BitMut<'a> {
        /* ... */
    }

    #[inline(always)]
    pub fn speed(&mut self) -> &mut u16 {
        /* ... */
    }
}
```

```rust
let mut pi = PiMut::new(&mut buf);

pi.btn_start() = true;
pi.speed() += 20;
```

##### Advantages
- Forced inlining and no intermediate representation enforces zero-costness.
- No concerns about passing around references to `&PiMut`

##### Disadvantages
- Method-syntax looks ugly and unnatural in the user code.
- Will need two completely separate structs for immutable/mutable access.

#### 3. svd2rust style
```rust
struct PiMut<'a> {
    pi: &mut 'a [u8],
}

impl<'a> PiMut<'a> {
    pub fn new() { /* ... */ }

    /* ... */
}
```

```rust
let mut pi = PiMut::new(&mut buf);

pi.btn_start().write(|w| w.set(true));
let speed = pi.speed().read();
```

#### Advantages
- Zero-cost enforcement.

#### Disadvantages
- Very verbose

## Misc. Design Questions
### Alignment
- Do we require the PI slice to come pre-aligned?
- Do we enforce the PI slice to be aligned by using `&[u32]`?
- Do we accept non-aligned PIs by just never making alignment assumptions?

=> Experimentation with godbolt seems to indicate that loading the values from
unaligned offsets is no slower than loading from aligned offsets.
