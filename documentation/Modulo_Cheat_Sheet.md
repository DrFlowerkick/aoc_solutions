# Modulo-Spickzettel (Cheat Sheet)

**Übersetzung der Notation:** `a ≡ b (mod m)` bedeutet: `a` und `b` lassen bei Division durch `m` denselben Rest.
In Code: `a % m == b % m`.

## 1) Grundrechenarten

Gilt immer, solange man das Ergebnis am Ende wieder modulo m nimmt:

- **Addition:** `(a + b) mod m = ((a mod m) + (b mod m)) mod m`
- **Subtraktion:** `(a - b) mod m = ((a mod m) - (b mod m)) mod m` *(Achtung bei negativen Ergebnissen: Nutze `rem_euclid` in Code!)*
- **Multiplikation:** `(a · b) mod m = ((a mod m) · (b mod m)) mod m`

## 2) Gleichungen auflösen

### Einfache Addition: `(x + y) mod m = n`

- **Lösung:** `x = (n - y) mod m`

### Multiplikation: `(x · y) mod m = n`

1. Berechne den größten gemeinsamen Teiler: `d = gcd(y, m)`
2. Wenn `d` die Zahl `n` **nicht** teilt $\rightarrow$ **Keine Lösung existent**.
3. Wenn doch: Teile die ganze Gleichung (inkl. Modulus `m`) durch `d`.
4. Finde das **Modulare Inverse** der neuen Zahl `y'` und multipliziere es auf beiden Seiten.

## 3) Modulares Inverses ("Division" im Modulo)

Das modulare Inverse von `a` ist die Zahl $a^{-1}$, für die gilt:
`a · a^{-1} ≡ 1 (mod m)`

- **Bedingung:** Existiert nur, wenn `gcd(a, m) = 1` (a und m sind teilerfremd).
- **Berechnung:** Über den *Erweiterten Euklidischen Algorithmus*.
- **Trick für Primzahlen:** Wenn `m` eine Primzahl ist, gilt laut dem kleinen Satz von Fermat:
  $a^{-1} \equiv a^{m-2} \pmod m$.

## 4) Chinesischer Restsatz (Mehrere Modulo-Bedingungen)

Wenn du ein `x` suchst für:
`x ≡ r1 (mod p1)`
`x ≡ r2 (mod p2)`
`x ≡ r3 (mod p3)`
(wobei p1, p2, p3 Primzahlen oder paarweise teilerfremd sind):

1. $M = p_1 \cdot p_2 \cdot p_3$
2. $M_i = M / p_i$ (für jede Zeile)
3. $y_i =$ Modulares Inverse von $M_i$ modulo $p_i$
4. **Lösung:** $x = (r_1 \cdot M_1 \cdot y_1 + r_2 \cdot M_2 \cdot y_2 + r_3 \cdot M_3 \cdot y_3) \pmod M$

## 5) Praxis-Tipps für Programmierer (z.B. Rust)

- **Negativer Modulo:** `-3 % 12` ergibt oft `-3`. Verwende `-3_i32.rem_euclid(12)`, um das korrekte mathematische Ergebnis `9` zu erhalten.
- **Riesige Potenzen (`a^e mod m`):** Niemals zuerst potenzieren! Nutze den *Square-and-Multiply*-Algorithmus (oft `modpow` genannt), der in jedem Zwischenschritt den Modulo zieht. Vermeidet Integer-Overflows und läuft in `O(log e)` Zeit.
