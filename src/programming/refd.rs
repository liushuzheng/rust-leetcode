static mut STASH: &i32 = &128;

fn f<'a>(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *s < *r {
            s = r;
        }
    }
    s
}