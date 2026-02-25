pub mod zk_guillou_quisquater;

//adição modular , subtração , multiplicação , divisão , power, gdc, modulo inverso
// u2 -> dois estados 0 ou 1
#[derive(Debug)]
pub struct Number {
    pub value: [bool; 32], //array que não cresce
}
const ZERO: Number = Number { value: [false; 32] };

pub fn bit_add(x: Number, y: Number) -> Number {
    let mut carry = false;
    let mut res = ZERO;
    for (i, (xi, yi)) in x.value.iter().zip(y.value.iter()).enumerate() {
        //transforma o valor numa lista ligada e devolta um tuplo com todos os valores da lista ligada
        match (*xi, *yi, carry) {
            (false, false, false) => res.value[i] = false, // (0,0,0 => 0)
            (false, true, false) | (true, false, false) => res.value[i] = true,
            (true, true, false) | (true, false, true) | (false, true, true) => {
                carry = true;
                res.value[i] = false;
            }
            (true, true, true) => {
                carry = true;
                res.value[i] = true;
            }
            (false, false, true) => {
                carry = false;
                res.value[i] = false;
            }
        }
    }
    res
}

//power , maior divisor comum , inverso multiplicativo
// //passar n para binario
//
pub fn power(x: u32, n: u32) -> u32 {
    let mut x = x;
    let mut n = n;
    let mut res = 1;
    while n > 0 {
        if (n & 1) == 1 {
            res *= x;
        }
        x *= x;
        n >>= 1;
    }
    res
}

pub fn mdv(mut a: u32, mut b: u32) -> u32 {
    let mut resto = 0;
    while b != 0 {
        resto = a % b;
        a = b;
        b = resto;
    }
    a
}

pub fn domodulus(x: u32, m: u32) -> u32 {
    x % m
}

pub fn eea(x: u32, m: u32) -> Option<u32> {
    if m == 0 {
        return None;
    } //nunca há inverso modular de 0 * inv ≡ 1
    if x == 0 {
        return None;
    }

    //para guardar o resto das divisões do algoritmo de euclides
    let mut x1: u32 = m; // m≡0(mod m)
    let mut y1: u32 = m;

    //coeficientes para o inverso
    let mut x2: u32 = x;
    let mut y2: u32 = 1;

    while x2 != 1 {
        // so quando chega a gcd(x,m)=1 é que existe inverso modular
        if x2 == 0 {
            return None;
        }

        let resto = x1 % x2; //x1=divisao⋅x2+resto
        let divisao = x1 / x2;

        //não podemos ter negativos logo vamos fazer a versão modular
        let prod = (((y2 as u64) * (divisao as u64)) % (m as u64)) as u32;
        let y = (((y1 as u64) + (m as u64) - (prod as u64)) % (m as u64)) as u32;
        // (y1+m−prod)mod m // y1 = 3, prod = 5, m = 7

        x1 = x2;
        x2 = resto;
        y1 = y2;
        y2 = y;
    }

    Some(domodulus(y2, m))
}

pub fn prime(n: u32) -> bool {
    let root = (n as f64).sqrt() as u32;

    for i in 2..=root {
        if n % i == 0 {
            return false;
        }
    }

    true
}

//é primo se for maior que 1
// e divisível apenas por 1 e por si próprio
