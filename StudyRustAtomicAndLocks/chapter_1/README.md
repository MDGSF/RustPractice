# chapter 1

Cell and RefCell are the standard types for single-threaded interior mutability. Atomics, Mutex,
and RwLock are their multi-threaded equivalents.

Cell and atomics only allow replacing the value as a whole, while RefCell, Mutex, and RwLock allow
you to mutate the value directly by dynamically enforcing access rules.

An RwLock or reader-writer lock is the concurrent version of a RefCell. An RwLock<T> holds a T and tracks any outstanding borrows.
However, unlike a RefCell, it does not panic on conflicting borrows. Instead, it blocks the current thread—(putting it to
sleep)—while waiting for conflicting borrows to disappear. We’ll just have to patiently wait for our turn with the data, after
the other threads are done with it.

