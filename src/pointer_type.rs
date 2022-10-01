

/// ! Shared reference (&)
/// These point to memory owned by some  other value.
/// when a shared reference to a value is created it prevents direct mutation 
/// of the value.
/// 
/// 
/// ! Mutable references (&mut)
/// These also point to memory owned by some other value.
/// A mutable reference type is written &mut type or &'a mut type. Amutable reference 
/// hasn't been borrowed.
/// 
/// 
/// ! Raw pointer (*const and *mut)
/// Raw pointers are pointers without safety or liveness guarantees. Raw pointers
/// are written as *const T or *mut T
/// 
/// 
/// ! Smart Pointers
/// The standard library contains additional `smart pointer` types beyond references and raw pointers.

fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn binop_works() {
        let mut x = add(5, 7);

        type Binop = fn(i32, i32) -> i32;
        let bo: Binop = add;
        x = bo(5, 7);

        assert_eq!(x, 12)
    }
}