use std::{
    error::Error,
    ops::{Add, Mul},
};

use rug::{Integer, integer::IsPrime};

pub struct State {
    pub p: Integer,
    pub q: Integer,
    pub v: Integer,
    pub identity: Integer,  //ja
    pub nonce: Integer,     //r -> número aleatorio
    pub challange: Integer, //e
}

impl State {
    pub fn guillou_quisquater(&self) -> Result<(), Box<dyn Error>> {
        match (&self.p.is_probably_prime(40), &self.q.is_probably_prime(40)) {
            (IsPrime::No, IsPrime::No) => return Err("not prime".into()),
            (IsPrime::Yes, IsPrime::No) => return Err("not prime".into()),
            (IsPrime::No, IsPrime::Yes) => return Err("not prime".into()),
            _ => (),
        }
        let n = self.p.clone().mul(&self.q);
        let phi = (self.p.clone().add(Integer::NEG_ONE)).mul(self.q.clone().add(Integer::NEG_ONE));

        let s = match self.v.clone().invert(&phi) {
            //interger::invert(self.v, phi)
            Ok(v) => v,
            _ => return Err("not possible".into()),
        }; //visivel para todos os utilizadores
        let mut secret = Integer::ONE.clone();
        let mut i = Integer::ONE.clone();
        while i <= s {
            //só quero percorrer x vezes
            secret = secret.mul(&self.identity).modulo(&n);
            i = i.add(Integer::ONE); // i += 1
        }
        secret = match secret.invert(&n) {
            Ok(secret) => secret,
            _ => return Err("não existe inverso modular".into()),
        };

        let mut commitment = Integer::ONE.clone();
        let mut i = Integer::ONE.clone();
        while i <= self.v {
            commitment = commitment.mul(&self.nonce).modulo(&n);
            i = i.add(Integer::ONE);
        }

        //challange tem de star entre 1 < e <= v
        let mut resposta = Integer::ONE.clone();
        let mut i = Integer::ONE.clone();
        while i <= self.challange {
            resposta = resposta.mul(&secret).modulo(&n);
            i = i.add(Integer::ONE);
        }
        resposta = resposta.mul(&self.nonce).modulo(&n);
        let mut z1 = Integer::ONE.clone();
        let mut i = Integer::ONE.clone();
        while i <= self.v {
            z1 = z1.mul(&resposta).modulo(&n);
            i = i.add(Integer::ONE);
        }

        let mut z2 = Integer::ONE.clone();
        let mut i = Integer::ONE.clone(); //* PARA DESREFERENCIAR , para transformar num valor e assumir valor
        while i <= self.challange {
            z2 = z2.mul(&self.identity).modulo(&n);
            i = i.add(Integer::ONE);
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

//is_probably prime : qunatas mais vezes executar menor é a probabilidade
