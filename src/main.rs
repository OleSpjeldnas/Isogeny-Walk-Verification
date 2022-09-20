// use library from winterfell
//extern crate winterfell;
use winterfell::math::{fields::f64::BaseElement, FieldElement, StarkField};
//use subtle::Choice;
//extern crate winterfell;
trait EvenElement {
    fn is_even(&self) -> bool;
    fn half_without_mod(&self) -> u64;
}
impl EvenElement for u64 {
    fn is_even(&self) -> bool {
        self % 2 == 0
     
    }
    fn half_without_mod(&self) -> u64 {
        self / 2
    }
}

trait SQrt {
    fn pow1(&self, exponent: u64, modulus: u64) -> u64;
    fn gcd(&self, other: u64) -> Self;
    fn order(&self, other: &Self) -> i32;
    fn convertx2e(&self) -> [Self; 2] where Self: Sized;
    fn sqrt(&self, modulus: u64) -> Option<u64>;
}
impl SQrt for u64 {
    fn pow1(&self, exponent: u64, modulus: u64) -> u64 {   
    if modulus == 1 { return 0 }
    let modulus = modulus as u128;
    let mut result = 1;
    let mut base = *self as u128;
    let mut exp = exponent;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        //println!("exp is {}", exp);
        //println!("base is {}", base);
        base = base * base % modulus
    }
    result as u64
        
    }
    fn gcd(&self, other: u64) -> Self {
        let mut a = *self;
        let mut b = other;
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
    fn order(&self, other: &Self) -> i32 {
        let s = *self;
        let t = *other;
        if s.gcd(t) != 1 {
            return -1;
        }
        let mut k = 1;
        loop {
            if t.pow1(k, s) == 1 {
                return k as i32;
            }
            k += 1;
        }
    }
    fn convertx2e(&self) -> [Self; 2] where Self: Sized {
        let mut x = *self;
        let mut e = 0;
        while x.is_even() {
            x = x.half_without_mod();
            e += 1;
        }
        [x, e]
    }
    fn sqrt(&self, modulus: u64) -> Option<u64>{
        if self.gcd(modulus) != 1 {
            return None;
        }
        if self.pow1((modulus-1)/2, modulus) != 1 {
            return None;
        }
        let [s, e] = (modulus-1).convertx2e();
        let mut q = 2u64;

        loop {
            if q.pow1((modulus-1)/2, modulus) == modulus-1 {
                break;
            }
            q += 1;}

            let mut x = self.pow1((s+1)/2, modulus);
            let mut b = self.pow1(s, modulus);
            let mut g = q.pow1(s, modulus);
            let mut r = e as u32;
            loop {
                let mut m = 0u32;
                while m < r {
                    if modulus.order(&b) == -1 {
                        return None
                    }
                    if modulus.order(&b) == 2u64.pow(m) as i32 {
                        break;
                    }

                    m += 1;}
                if m == 0 {
                    return Some(x)
                }
                x = ((x as u128) * (g.pow1(2u64.pow(r-m-1), modulus) as u128) % (modulus as u128)) as u64;
                g = g.pow1(2u64.pow(r-m), modulus);
                b = ((b as u128) * (g as u128) % (modulus) as u128) as u64;

                if b == 1 {
                    return Some(x)
                }
                r = m;

            }
        }

    }

trait Sqrt {
    fn pow1(&self,exponent: u64, modulus: u64) -> u64;
    fn gcd(&self, other: &Self) -> Self;
    fn order(&self) -> u64;
    fn convertx2e(&self) -> [u64;2];
    fn sqrt(&self) -> Option<BaseElement>;
}
impl Sqrt for BaseElement {
    fn pow1(&self, exponent: u64, modulus: u64) -> u64 {
        let modulus = modulus as u128;
        let mut result = 1;
        let mut base = self.as_int() as u128;
        base = base % modulus;
        let mut ex = exponent;
        
        while ex > 0 {
            if ex.is_even() {
                result = (result * base) % modulus;
            } 
            base = (base * base) % modulus;
            ex = ex.half_without_mod();
            
        }
        result as u64
    }
    fn gcd(&self, other: &Self) -> Self {
        if other.as_int() == 0 {
            return *self;
        }
        self.gcd(&BaseElement::new(self.as_int() % other.as_int()))
    }
    fn order(&self) -> u64 {
        //const MODULUS: u64 = 2u64.pow(64) - 2u64.pow(32) + 1;
        let mut k = 3u64;
        loop {
            if self.exp(k).as_int() == 1 {
                return k;
            }
            k += 1;
        }
}
fn convertx2e(&self) -> [u64;2] {
    let mut e = 0u64;
    let mut x = *self;
    while x.as_int().is_even() {
        x = BaseElement::new(x.as_int().half_without_mod());
        e += 1;
    }
    [x.as_int(), e]
}
    fn sqrt(&self) -> Option<BaseElement> {
        const MODULUS: u64 = 2u64^(64) - 2u64^(32) + 1;
        if self.pow1((MODULUS-1)/2, MODULUS) == MODULUS - 1 {
            return None;
        }
        if self.as_int().gcd(MODULUS) != 1 {
            return None;
        }
        let [s, e] = BaseElement::new(MODULUS-1).convertx2e();
        let mut q = BaseElement::new(2);
        loop {
            //println!("q = {}", q.as_int());
            if q.pow1((MODULUS-1)/2, MODULUS) == MODULUS - 1 {
                break;
            }
            q += BaseElement::new(1);
        }
        let mut x = self.exp((s+1)/2);
        let mut b = self.exp(s);
        let mut g = q.exp(s);
        let mut r = e;
        loop {
            println!("hello");
            let mut m = 0;
            while m < r {
                if MODULUS.order(&b.as_int()) == -1 {
                    println!("minus one");
                    return None
                }
                if MODULUS.order(&b.as_int()) as u64 == 2^m{
                    break;
                }
                m += 1;
            }
            if m == 0 {
                return Some(x);
            }
            //assert!(m < r);
            x = x*BaseElement::new(g.pow1(2^((BaseElement::new(r)-BaseElement::new(m)-BaseElement::new(1)).as_int()), MODULUS));
            g = BaseElement::new(g.pow1(2^(r-m), MODULUS));
            b *= g;
            if b.as_int() == 1 {
                return Some(x);
            }
            r = m;
        }

    }   
}

fn main() {
    const MODULUS: u64 = 0xFFFFFFFF00000001;
    const J_MINUS: BaseElement = BaseElement::new(1728);
    const J_0: BaseElement = BaseElement::new(287496);
    let j_0_2 = J_0.square();
    let j_0_3: BaseElement = J_0.exp(3); 
    let j_0_4: BaseElement = J_0.exp(4);
    let j_minus_2 = J_MINUS.square();

    let d = j_0_4 - BaseElement::new(2976)*j_0_3+BaseElement::new(2532192)*j_0_2-BaseElement::new(2976)*J_0*J_MINUS-BaseElement::new(645205500)*J_0-BaseElement::new(3)*j_minus_2+BaseElement::new(324000)*J_MINUS- BaseElement::new(8748000000);
    //let t: BaseElement = BaseElement::new(4611686018427387904).square();
    //println!("{}", BaseElement::new(100).sqrt().unwrap().as_int());  
    println!("{}", 1000u64.sqrt(MODULUS).unwrap()); 
    //println!("{}", MODULUS);
}

