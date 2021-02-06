use std::slice;

fn main() {
    // 裸指针
    let raw_p: *const u32 = &10;

    unsafe {
        assert!(*raw_p == 10);
    }

    let some_vector = vec![1, 2, 3, 4];

    let pointer = some_vector.as_ptr();
    let length = some_vector.len();
    
    // 调用不安全函数
    unsafe {
        let my_slice: &[u32] = slice::from_raw_parts(pointer, length);

        assert_eq!(some_vector.as_slice(), my_slice);
    }
}
