use memchr::{memchr, memchr2};

fn main() {
    let _x1 = memchr(b'x', b"abcx");
    let _x2 = memchr2(b'x', b'y', b"abcx");
}
