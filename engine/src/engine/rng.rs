use std::cell::{RefCell};

// https://users.rust-lang.org/t/random-number-without-using-the-external-crate/17260/11
const KX: u32 = 123456789;
const KY: u32 = 362436069;
const KZ: u32 = 521288629;
const KW: u32 = 88675123;

thread_local! {
    static RNG: RefCell<Rand> = RefCell::new(Rand::new(0));
}

pub fn uid() -> u32 {
    RNG.with(|rng| -> u32 {
        rng.borrow_mut().rand_range(1, i32::MAX) as u32
    })
}

struct Rand {
    x: u32, y: u32, z: u32, w: u32
}

impl Rand{
    fn new(seed: u32) -> Rand {
        Rand{
            x: KX^seed, y: KY^seed,
            z: KZ, w: KW
        }
    }

    // Xorshift 128, taken from German Wikipedia
    fn rand(&mut self) -> u32 {
        let t = self.x^self.x.wrapping_shl(11);
        self.x = self.y; self.y = self.z; self.z = self.w;
        self.w ^= self.w.wrapping_shr(19)^t^t.wrapping_shr(8);
        return self.w;
    }

    fn shuffle<T>(&mut self, a: &mut [T]) {
        if a.len()==0 {return;}
        let mut i = a.len()-1;
        while i>0 {
            let j = (self.rand() as usize)%(i+1);
            a.swap(i,j);
            i-=1;
        }
    }

    fn rand_range(&mut self, a: i32, b: i32) -> i32 {
        let m = (b-a+1) as u32;
        return a+(self.rand()%m) as i32;
    }

    fn rand_float(&mut self) -> f64 {
        (self.rand() as f64)/(<u32>::max_value() as f64)
    }
}

