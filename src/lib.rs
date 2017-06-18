#![no_std]


#[doc(hidden)]
pub use core::mem::transmute;

/**
 * Slices (via the Index trait & operation) into fixed size arrays
 *
 * Will panic with the same rules as normal slicing.
 *
 * Will not compile if bounds are not static.
 *
 * Will not compile if end bound proceeds start bound.
 *
 * # Format
 *
 * ```notest
 * index_fixed! ( {&,&mut} <slice> ; .. <end>)
 * index_fixed! ( {&,&mut} <slice> ; <start> , .. <end>)
 * index_fixed! ( {&,&mut} <slice> ; <start> , ... <end>)
 * ```
 *
 * # Examples
 *
 * ```
 * #[macro_use]
 * extern crate index_fixed;
 *
 * fn main() {
 *   let my_slice = [1, 2, 3, 4];
 *   let slice_of_2 = index_fixed!(&my_slice ; .. 2);
 *   assert_eq!(slice_of_2, &my_slice[..2]);
 * }
 * ```
 */
// FIXME example test disabled because index_fixed!() is not defined
#[macro_export]
macro_rules! index_fixed {
    (&mut $s:expr ;  .. $e:expr) => {
        index_fixed!(&mut $s; 0 , .. $e )
    };
    (&mut $s:expr ; $b:expr , ... $e:expr) => {
        index_fixed!(&mut $s; $b , .. ($e + 1))
    };
    (&mut $s:expr ; $b:expr , .. $e:expr) => { {
        fn conv<T>(a: &mut[T]) -> &mut[T;$e - $b] {
            let b = &mut a[$b..$e];
            unsafe { $crate::transmute::<*mut T, &mut[T;$e - $b]>(b.as_mut_ptr()) }
        }
        conv(&mut $s)
    } };
    (& $s:expr ; .. $e:expr) => {
        index_fixed!(& $s ; 0 , .. $e)
    };
    (& $s:expr ; $b:expr , ... $e:expr) => {
        index_fixed!(& $s ; $b , .. ($e + 1))
    };
    (& $s:expr ; $b:expr , .. $e:expr) => { {
        fn conv<T>(a: &[T]) -> &[T;$e - $b] {
            let b = &a[$b..$e];
            unsafe { $crate::transmute::<*const T, &[T;$e - $b]>(b.as_ptr()) }
        }
        conv(& $s)
    } };
}

/**
 * `slice::get` and `slice::get_mut`, but return an `Option<&[T;N]>` or `Option<&mut [T;N]>`
 *
 * Will not compile if bounds are not static.
 *
 * Will not compile if end bound proceeds start bound.
 *
 * # Format
 *
 * ```notest
 * index_fixed_get! ( {&,&mut} <slice> ; .. <end>)
 * index_fixed_get! ( {&,&mut} <slice> ; <start> , .. <end>)
 * index_fixed_get! ( {&,&mut} <slice> ; <start> , ... <end>)
 * ```
 *
 * # Examples
 *
 * ```
 * #[macro_use]
 * extern crate index_fixed;
 *
 * fn main() {
 *   let my_slice = [1, 2, 3, 4];
 *   let slice_of_2 = index_fixed_get!(&my_slice ; .. 2);
 *   assert_eq!(slice_of_2, Some(&[1,2]));
 * }
 * ```
 */
#[macro_export]
macro_rules! index_fixed_get {
    (&mut $s:expr ;  .. $e:expr) => {
        index_fixed_get!(&mut $s; 0 , .. $e )
    };
    (&mut $s:expr ; $b:expr , ... $e:expr) => {
        index_fixed_get!(&mut $s; $b , .. ($e + 1))
    };
    (&mut $s:expr ; $b:expr , .. $e:expr) => { {
        fn conv<T>(a: &mut[T]) -> Option<&mut[T;$e - $b]> {
            a.get_mut($b..$e).map(|b|
                unsafe { $crate::transmute::<*mut T, &mut [T;$e - $b]>(b.as_mut_ptr()) }
            )
        }
        conv(&mut $s)
    } };
    (& $s:expr ; .. $e:expr) => {
        index_fixed_get!(& $s ; 0 , .. $e)
    };
    (& $s:expr ; $b:expr , ... $e:expr) => {
        index_fixed_get!(& $s ; $b , .. ($e + 1))
    };
    (& $s:expr ; $b:expr , .. $e:expr) => { {
        fn conv<T>(a: &[T]) -> Option<&[T;$e - $b]> {
            a.get($b..$e).map(|b|
                unsafe { $crate::transmute::<*const T, &[T;$e - $b]>(b.as_ptr()) }
            )
        }
        conv(& $s)
    } };
}
