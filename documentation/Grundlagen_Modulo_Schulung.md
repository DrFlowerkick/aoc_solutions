# Grundlagenschulung: Rechnen modulo m (Division mit Rest)

**Zielgruppe:** Schüler:innen eines Gymnasiums und Studierende im Grundstudium.  
**Lernziele:** Lineare Kongruenzen lösen, modulare Inversen finden und zusammengesetzte Aufgaben sicher bearbeiten.

---

## 1) Was bedeutet „modulo“?
Wir schreiben `a ≡ b (mod m)`, wenn `a` und `b` beim Teilen durch `m` den gleichen Rest haben.  
Beispiel: `17 ≡ 5 (mod 12)`, denn `17 = 1·12 + 5` und `5 = 0·12 + 5`.

**Regeln (jeweils modulo m):**
- `(a + b) mod m ≡ (a mod m + b mod m) mod m`
- `(a − b) mod m ≡ (a mod m − b mod m) mod m`
- `(a · b) mod m ≡ (a mod m · b mod m) mod m`

**Wichtig:** „Division“ gibt es nur, wenn ein **modulares Inverses** existiert.

---

## 2) Lösen von `(x + y) mod m = n`
Interpretation: Beide Seiten sind Reste `0,…,m−1`. Die Kongruenz lautet  
`x + y ≡ n (mod m)`.

**Nach x auflösen:**  
`x ≡ n − y (mod m)` ⇒ **kanonisch:** `x = (n − y) mod m`.

**Beispiel:** `(x + 9) mod 12 = 3` ⇒ `x ≡ 3 − 9 ≡ −6 ≡ 6 (mod 12)` ⇒ `x = 6`.

---

## 3) Lösen von `(x · y) mod m = n` (lineare Kongruenz)
Schreibe `y·x ≡ n (mod m)` – eine lineare Kongruenz in `x`.

**Vorgehen:**
1. `d = gcd(y, m)` berechnen.
2. Falls `d ∤ n` ⇒ **keine Lösung**.
3. Sonst reduzieren: `y' = y/d`, `m' = m/d`, `n' = n/d`. Dann gilt `gcd(y', m') = 1`.
4. **Modulares Inverses** `inv = (y')^{-1} (mod m')` bestimmen.
5. Basislösung: `x0 ≡ n' · inv (mod m')`.
6. **Alle** Lösungen modulo `m`: `x = x0 + k·m'` für `k = 0,1,…,d−1`.

**Beispiel:** `(x · 6) mod 14 = 4`  
`d = gcd(6,14)=2` teilt `4` ⇒ lösbar. `y'=3, m'=7, n'=2`.  
Inverse von `3` mod `7` ist `5` (`3·5≡1`).  
`x0 ≡ 2·5 ≡ 10 ≡ 3 (mod 7)`. **Alle Lösungen:** `x ∈ {3, 10}` modulo `14`.

---

## 4) Umgang mit `1 / ((x + y) mod m) = n`
In modularer Arithmetik heißt „`1 / z`“ das **modulare Inverse** von `z` (falls existent).  
Formal: `1 / z = n (mod m)` bedeutet `z · n ≡ 1 (mod m)`.

Setze `z = (x + y) mod m`. Dann gilt:  
`((x + y) mod m) · n ≡ 1 (mod m)` ⇒ `n·x ≡ 1 − n·y (mod m)`.

Das ist wieder eine lineare Kongruenz in `x` mit Koeffizient `n`:
- **Existenz:** Lösung genau dann, wenn `gcd(n, m)` den rechten Rest `1 − n·y` teilt.
- **Spezialfall `gcd(n, m)=1`:** Immer eindeutig modulo `m`:  
  `x ≡ (1 − n·y) · n^{-1} (mod m)`.

**Beispiel:** `1/((x+4) mod 11) = 3 (mod 11)`  
⇔ `((x+4) mod 11) · 3 ≡ 1` ⇔ `3x + 12 ≡ 1` ⇔ `3x ≡ −11 ≡ 0 (mod 11)`  
⇒ `x ≡ 0 (mod 11)`.

---

## 5) Zusammengesetzte Aufgaben: `x = (a + b) mod m` und `y = x^2 mod n`
In Programmen ist `x` meist der **kanonische Rest** `0,…,m−1`. Dann ist `y` eindeutig: `y = (x^2) mod n`.

**Beispiel:** `a=23, b=19, m=12, n=7`  
`x = (23+19) mod 12 = 42 mod 12 = 6`  
`y = 6^2 mod 7 = 36 mod 7 = 1`.

**Theorie-Hinweis:** Kennst du nur `x ≡ x0 (mod m)` als Klasse (ohne Vertreter), dann sind alle Kandidaten `x0 + k·m`.  
Für `y = x^2 mod n` können dadurch **verschiedene** Reste entstehen. Die Menge ist periodisch in  
`k` mit Periode `T = n / gcd(m, n)`. Um alle möglichen `y` zu sehen, reicht `k=0,…,T−1`.

---

## 6) Modulares Inverses in der Praxis
Das Inverse `a^{-1} (mod m)` existiert **genau dann**, wenn `gcd(a, m) = 1`.  
Man findet es mit dem **erweiterten Euklidischen Algorithmus**.

**Mini-Beispiel:** Inverses von `8 mod 29`?  
`gcd(8,29)=1`. Ergebnis: `11`, denn `8·11=88≡1 (mod 29)`.

---

## 7) Häufige Stolpersteine
- **„Durch Null teilen“:** Wenn `z ≡ 0 (mod m)`, existiert kein Inverses.
- **gcd-Bedingung vergessen:** Für `y·x ≡ n (mod m)` muss `gcd(y, m)` den `n` teilen.
- **Zwischenschritte nicht modulo m reduziert.**
- **Negative Reste:** `−3 mod 12 = 9`. In vielen Sprachen brauchst du den nichtnegativen Rest, z. B. Rust `rem_euclid`.

---

## 8) Übungen (mit Lösungen)

**(A)** Löse nach `x`: `(x + 17) mod 23 = 5`.  
**(B)** Löse nach `x`: `(9·x) mod 28 = 8`.  
**(C)** Interpretiere und löse: `1/((x + 6) mod 13) = 4 (mod 13)`.  
**(D)** Gegeben: `x = (a + b) mod 15`, mit `a=41`, `b=29`; `n=8`. Berechne `y = x^2 mod n`.

**Lösungen:**  
(A) `x ≡ 5 − 17 ≡ −12 ≡ 11 (mod 23)` ⇒ `x = 11`.  
(B) `gcd(9,28)=1` ⇒ `inv(9)≡25` ⇒ `x ≡ 8·25 ≡ 200 ≡ 4 (mod 28)`.  
(C) `((x+6) mod 13)·4 ≡ 1` ⇒ `4x + 24 ≡ 1` ⇒ `4x ≡ −23 ≡ 3 (mod 13)` ⇒ `x ≡ 3·10 ≡ 30 ≡ 4 (mod 13)`.  
(D) `x = (41+29) mod 15 = 70 mod 15 = 10`. `y = 10^2 mod 8 = 100 mod 8 = 4`.

---

## 9) Kurzzusammenfassung (Cheat Sheet)
- `(x + y) mod m = n` ⇒ `x ≡ n − y (mod m)` ⇒ `x = (n − y) mod m`.
- `(x · y) mod m = n`:
  - `d=gcd(y,m)`; wenn `d ∤ n` → keine Lösung.
  - sonst reduzieren (`/d`), Inverses von `y'` modulo `m'` und `x0 ≡ n'·(y')^{-1}`; alle Lösungen `x = x0 + k·m'`.
- `1 / z (mod m)` bedeutet `z^{-1} (mod m)` existiert (`gcd(z,m)=1`) und `z·z^{-1} ≡ 1`.
- `1 / ((x+y) mod m) = n` ⇒ `(x+y)·n ≡ 1 (mod m)` ⇒ lineare Kongruenz in `x`.
- `x = (a + b) mod m`, `y = x^2 mod n`: erst kanonisches `x`, dann `y` berechnen. Bei Klassen: Periode `T = n / gcd(m, n)`.

---

## 10) Beispiel: `modpow` in Rust (schnelles Potenzieren modulo m)

**Idee:** Statt `a^e` riesig zu machen und dann `mod m` zu nehmen, nutzt man **Square-and-Multiply**.
Das arbeitet in `O(log e)`-Schritten und reduziert zwischendurch immer wieder modulo `m`.

### Variante für `u64` (ohne externe Crates)
```rust
/// (base^exp) % modulus in O(log exp) mit Overflow-Schutz via u128.
pub fn modpow_u64(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    assert!(modulus != 0, "modpow: modulus must be > 0");
    if modulus == 1 { return 0; }
    let mut result = 1 % modulus;
    base %= modulus;
    while exp > 0 {
        if (exp & 1) == 1 {
            result = ((result as u128 * base as u128) % (modulus as u128)) as u64;
        }
        base = ((base as u128 * base as u128) % (modulus as u128)) as u64;
        exp >>= 1;
    }
    result
}

fn main() {
    // Klassisches Beispiel: 7^560 mod 561 = 1
    // (561 ist eine Carmichael-Zahl; hier liefert modpow korrekt 1)
    let r = modpow_u64(7, 560, 561);
    println!("{r}"); // 1
}
```

### Variante mit großen Zahlen (`num-bigint`)
```toml
# Cargo.toml
[dependencies]
num-bigint = "0.4"
num-traits = "0.2"
```
```rust
use num_bigint::BigUint;
use num_traits::FromPrimitive;

fn main() {
    let a = BigUint::from_u64(7).unwrap();
    let e = BigUint::from_u64(560).unwrap();
    let m = BigUint::from_u64(561).unwrap();
    let r = a.modpow(&e, &m);
    println!("{r}"); // 1
}
```

> **Tipp (Inverses bei primem Modulus):** Ist `m` **prim** und `gcd(a,m)=1`, dann ist  
> `a^{-1} ≡ a^{m-2} (mod m)` (Fermat). Beispiel in Rust:
> ```rust
> let m: u64 = 1_000_000_007;
> let inv_42 = modpow_u64(42, m - 2, m); // Inverses von 42 modulo m
> ```
