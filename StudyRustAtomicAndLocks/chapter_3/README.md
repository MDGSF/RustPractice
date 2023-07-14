# chapter 3

Release and acquire memory ordering are used in a pair to form a happens-before relationship between threads.
Release memory ordering applies to store operations,
while Acquire memory ordering applies to load operations.

- load 只能用：`Acquire`, `SeqCst` or `Relaxed`。
- store 只能用：`Release`, `SeqCst` or `Relaxed`。

## fence

Using a separate fence can result in an extra processor instruction, though, which can be slightly less efficient.

More importantly, unlike a release-store or an acquire-load, a fence is not tied to any single atomic variable. This means that a
single fence can be used for multiple variables at once.

### Release store

The store of a release-acquire relationship,

```rust
a.store(1, Release);
```

can be substituted by a release fence followed by a relaxed store:

```rust
fence(Release);
a.store(1, Relaxed);
```

### Acquire load

The load of a release-acquire relationship,

```rust
a.load(Acquire);
```

can be substituted by a relaxed load followed by an acquire fence:

```rust
a.load(Relaxed);
fence(Acquire);
```


