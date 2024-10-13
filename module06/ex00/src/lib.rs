use std::ptr;

fn ft_swap<T>(a: &mut T, b: &mut T) {
    unsafe {
        let tmp = ptr::read(b);
        ptr::write(b, ptr::read(a));
        ptr::write(b, tmp);
    }
}

unsafe fn ft_strlen(s: *const u8) -> usize {

}
unsafe fn ft_strcpy(dst: *mut u8, src: *const u8) {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = String::from("Hello, World!");
        let mut b = String::from("Goodby, World!");
        ft_swap(&mut a, &mut b);
        assert_eq!(a, "Goodby, World!");
        assert_eq!(b, "Hello, World!");

        let s = b"Hello, World!\0";
        // SAFETY:
        //  /* ... */
        let len = unsafe { ft_strlen(s.as_ptr()) };
        assert_eq!(len, 13);

        let mut dst = [0u8; 14];
        // SAFETY:
        //  /* ... */
        unsafe { ft_strcpy(dst.as_mut_ptr(), s.as_ptr()) };
        assert_eq!(&dst, b"Hello, World!\0");
    }
}
