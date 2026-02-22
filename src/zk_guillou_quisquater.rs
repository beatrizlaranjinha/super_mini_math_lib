use std::error::Error;

use crate::{inverso_modular, prime};

pub struct State {
    pub p: u32,
    pub q: u32,
    pub v: u32,
    pub identity: u32,
    pub nonce: u32,
    pub challange: u32,
}

impl State {
    pub fn guillou_quisquater(&self) -> Result<(), Box<dyn Error>> {
        if !prime(self.p) || !prime(self.q) {
            return Err("não são primos!".into());
        }
        let n = self.p * self.q;
        let phi = (self.p - 1) * (self.q - 1);
        let s = match inverso_modular(self.v, phi) {
            Some(s) => s,
            None => return Err("não existe inversão modular aquii!".into()),
        }; //visivel para todos os utilizadores
        let mut secret = 1;
        for _ in 1..=s {
            //só quero percorrer x vezes
            secret = (secret * self.identity) % n;
        }
        secret = match inverso_modular(secret, n) {
            Some(secret) => secret,
            None => return Err("não existe inverso modular".into()),
        };
        let mut commitment = 1;
        for _ in 1..=self.v {
            commitment = commitment * self.nonce % n;
        }
        //challange tem de star entre 1 < e <= v
        let mut resposta = 1;
        for _ in 1..=self.challange {
            resposta = resposta * secret % n;
        }
        resposta = resposta * self.nonce % n;
        let mut z1 = 1;
        for _ in 1..=self.v {
            z1 = z1 * resposta % n;
        }

        let mut z2 = 1;
        for _ in 1..=self.challange {
            z2 = z2 * self.identity % n;
        }
        let z = z1 * z2 % n;
        println!("o valor de z é: {} ", z);
        println!("o valor de x é: {} ", commitment);

        if z == commitment {
            println!("x é igual a z");
            println!("B autenticou A");
        } else {
            println!("z não é igual a x");
            println!("B não autenticou A");
        }
        Ok(())
    }
}
