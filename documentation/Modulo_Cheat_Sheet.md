# Modulo-Spickzettel (Cheat Sheet)

**Grundidee:** `a ≡ b (mod m)` ⇔ gleiche Reste bei Division durch `m`.

**Regeln:**  
`(a±b) mod m = ((a mod m) ± (b mod m)) mod m`  
`(a·b) mod m = ((a mod m)·(b mod m)) mod m`

## 1) `(x + y) mod m = n`
`x ≡ n − y (mod m)` ⇒ **kanonisch:** `x = (n − y) mod m`.

## 2) `(x · y) mod m = n`
`d=gcd(y,m)`  
- Wenn `d ∤ n` → **keine Lösung**.  
- Sonst: `y'=y/d`, `m'=m/d`, `n'=n/d`, `gcd(y',m')=1`.  
  `x0 ≡ n'·(y')^{-1} (mod m')`.  
  **Alle Lösungen:** `x = x0 + k·m'` (`k=0,…,d−1`).

## 3) `1 / ((x + y) mod m) = n`
Bedeutet: `((x+y) mod m) · n ≡ 1 (mod m)`  
⇔ `n·x ≡ 1 − n·y (mod m)` ⇒ lineare Kongruenz in `x`.

## 4) `x = (a + b) mod m`, `y = x^2 mod n`
Erst kanonisches `x` berechnen, dann `y = (x^2) mod n`.  
Wenn nur Klasse `x ≡ x0 (mod m)` bekannt: mögliche `y` periodisch in `k` mit `T = n / gcd(m,n)`.

**Modulares Inverses:** existiert ⇔ `gcd(a,m)=1`. Finde es mit erweitertem Euklid.  
**Negativer Rest:** in vielen Sprachen positiv normalisieren (z. B. Rust `rem_euclid`).

## 5) `modpow` – schnelles Potenzieren modulo m
- Berechnet `a^e mod m` in `O(log e)` (Square-and-Multiply).
- Rust (u64):
  ```rust
  pub fn modpow_u64(mut a:u64, mut e:u64, m:u64)->u64{
      assert!(m!=0); if m==1{return 0;} let mut r=1%m; a%=m;
      while e>0{ if e&1==1{ r=((r as u128*a as u128)%(m as u128)) as u64; }
                 a=((a as u128*a as u128)%(m as u128)) as u64; e>>=1; }
      r
  }
  // Beispiel: 7^560 mod 561 = 1
  ```
- BigInt: `num_bigint::BigUint::modpow(&a,&e,&m)`.
- Inverses bei **primem** `m`: `a^{m-2} mod m` (Fermat).
