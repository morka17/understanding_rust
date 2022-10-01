/// ! Interior Mutability
/// A type has interior mutabilt if its internal state can be changed
/// through a shared reference to it.
///
/// std::cell::UnsafeCell<T> type is the only allowed way in Rust to disable this requirement. When UnsafeCell<T>
/// is immutably aliased, it is still safe to mutate, or obtain a mutable reference to,
///  the T it contains. As with all other types, it is undefined behavior to have multiple &mut UnsafeCell<T> aliases.
///
/// Example     
use std::cell::{RefCell, UnsafeCell};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{hint, thread};

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn understanding_unsafeCell() {
        let x: UnsafeCell<i32> = 42.into();

        let (p1, p2): (&UnsafeCell<i32>, &UnsafeCell<i32>) = (&x, &x);

        unsafe {
            let p1_exclusive: &mut i32 = &mut *p1.get(); // borrow
            *p1_exclusive += 27;
        }

        unsafe {
            // SAFETY: within this scope nobody expects to have exclusive access to `x`'s contents,
            // so we can have multiple shared accesses concurrently.
            let p2_shared: &i32 = &*p2.get();
            assert_eq!(*p2_shared, 42 + 27);
            let p1_shared: &i32 = &*p1.get();
            assert_eq!(*p1_shared, *p2_shared);
        }
    }

    #[test]
    fn understanding_refcell() {
        use std::cell::RefCell;
        let cell = RefCell::new(5);
        let old_value = cell.replace(6);
        assert_eq!(old_value, 5);
        assert_eq!(cell, RefCell::new(6));
    }

    #[test]
    fn understanding_atomic() {
        let spinlock = Arc::new(AtomicUsize::new(1));

        let spinlock_clone = Arc::clone(&spinlock);
        let thread = thread::spawn(move ||{
            spinlock_clone.store(0, Ordering::SeqCst);
        });

        // Wait for the other thread to release the lock
        while spinlock.load(Ordering::SeqCst) != 0 {
            hint::spin_loop();
        }

        if let Err(panic) = thread.join() {
            println!("Thread has an error: {panic:?}");
        }
    }

}
