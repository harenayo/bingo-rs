use super::{
    super::Caller,
    ArrayCaller,
};
use rand::{
    thread_rng,
    Rng as _,
};
use std::convert::identity;

#[test]
fn test() {
    let mut rng = thread_rng();

    for _ in 0..0xFF {
        let mut caller: ArrayCaller<75> = rng.gen();
        let mut generated = [false; 75];

        while let Option::Some(number) = caller.call() {
            let index = number as usize - 1;
            assert!(!generated[index]);
            generated[index] = true;
        }

        assert!(generated.into_iter().all(identity));
    }
}
