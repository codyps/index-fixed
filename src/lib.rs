#![no_std]

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
 * ```notest
 * let my_slice = [1, 2, 3, 4];
 * let slice_of_2 = index_fixed!(&my_slice ; .. 2);
 * assert_eq!(slice_of_2, &[1..2]);
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
        unsafe fn conv<T>(a: &mut[T]) -> &mut[T;$e - $b] {
            ::core::mem::transmute::<*mut T, &mut[T;$e - $b]>(a.as_mut_ptr())
        }
        unsafe { conv(&mut $s[$b..$e]) }
    } };
    (& $s:expr ; .. $e:expr) => {
        index_fixed!(& $s ; 0 , .. $e)
    };
    (& $s:expr ; $b:expr , ... $e:expr) => {
        index_fixed!(& $s ; $b , .. ($e + 1))
    };
    (& $s:expr ; $b:expr , .. $e:expr) => { {
        unsafe fn conv<T>(a: &[T]) -> &[T;$e - $b] {
            ::core::mem::transmute::<*const T, &[T;$e - $b]>(a.as_ptr())
        }
        unsafe { conv(& $s[$b..$e]) }
    } };
}

#[cfg(test)]
mod tests {
    #[test]
    fn const_to() {
        let a = [1u8,2,3,6];
        {
            let b : &[u8;1] = index_fixed!(&a; ..1);
            assert_eq!(b, &[1]);
        }

        {
            let b : &[u8;2] = index_fixed!(&a; ..2);
            assert_eq!(b, &[1, 2]);
        }
    }

    #[test]
    fn mut_to() {
        let mut a = [1u8,2,3,6];
        {
            let b : &mut [u8;2] = index_fixed!(&mut a; ..2);
            assert_eq!(b, &[1, 2]);

            b[1] = 5;
        }

        assert_eq!(a[1], 5);
    }

    #[test]
    fn const_range() {
        let a = [1u8,2,3,6];
        {
            let b : &[u8;2] = index_fixed!(&a; 1 * 2, .. 6 - 2);
            assert_eq!(b, &[3, 6]);
        }
    }

    #[test]
    fn mut_range() {
        let mut a = [1u8,2,3,6];
        {
            let b : &mut [u8;2] = index_fixed!(&mut a; 4/2, .. 2 + 2);
            assert_eq!(b, &[3, 6]);

            b[0] = 5;
        }
        assert_eq!(a[2], 5);
    }


    #[test]
    fn const_range_inc() {
        let a = [1u8,2,3,6];
        {
            let b : &[u8;3] = index_fixed!(&a; 1, ... 3);
            assert_eq!(b, &[2, 3, 6]);
        }
    }

    #[test]
    fn mut_range_inc() {
        let mut a = [1u8,2,3,6];
        {
            let b : &mut [u8;3] = index_fixed!(&mut a; 1, ... 3);
            assert_eq!(b, &[2, 3, 6]);

            b[0] = 5;
        }
        assert_eq!(a[1], 5);
    }

    #[test]
    fn type_infer() {
        let a = [1u8, 7, 19];
        let b = index_fixed!(&a; 1,..2);
        assert_eq!(&a[1..2], &b[..]);
    }
}
