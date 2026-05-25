# Grundlagenschulung: Rechnen modulo m (Die Mathematik der Uhren)

**Zielgruppe:** Anfänger, Schüler:innen und Studierende, die bisher nur den Basis-Modulo-Operator (`%` in der Programmierung) kennen.  
**Lernziele:** Die mathematische Schreibweise verstehen, lineare Kongruenzen lösen, modulare Inversen (Division) begreifen und mehrere Modulo-Bedingungen gleichzeitig lösen.

---

## 1) Was bedeutet „modulo“ in der Mathematik?

In der Programmierung ist `a % m` eine Operation, die dir den Rest einer Division ausgibt (z. B. `17 % 12 = 5`). In der Mathematik betrachtet man Modulo jedoch oft als einen **Raum** oder eine **Klasse** von Zahlen, die sich alle gleich verhalten.

Hier begegnet dir oft diese ungewohnte Schreibweise:
`a ≡ b (mod m)`

**Was bedeutet das übersetzt?**
Das Zeichen `≡` nennt man **Kongruenz**. Es ist fast wie ein Gleichheitszeichen `=`, bedeutet aber: *"a und b haben denselben Rest, wenn man sie durch m teilt"*.
In Programmier-Logik ausgedrückt heißt `a ≡ b (mod m)` einfach:
`a % m == b % m`

**Ein anschauliches Beispiel (Die Uhr):**
`17 ≡ 5 (mod 12)`
Wenn es 17:00 Uhr ist, zeigt der Zeiger auf die 5. Beide Zahlen landen im „12er-System“ auf derselben Position.

**Die wichtigsten Grundregeln:**
Du kannst in der Modulo-Welt ganz normal plus, minus und mal rechnen. Du musst nur aufpassen, dass du (wenn die Zahl zu groß wird) am Ende wieder den Rest ziehst.

- **Addition:** `(a + b) mod m ≡ (a mod m + b mod m) mod m`
- **Subtraktion:** `(a − b) mod m ≡ (a mod m − b mod m) mod m`
- **Multiplikation:** `(a · b) mod m ≡ (a mod m · b mod m) mod m`

---

## 2) Das Rätsel der „Division“: Modulares Inverses

In der normalen Mathematik ist die Division das Gegenteil der Multiplikation. Wenn `3 * x = 15`, teilen wir durch 3 und erhalten `x = 5`.
In der Modulo-Welt **gibt es keine klassische Division**. Stattdessen nutzen wir das sogenannte **Modulare Inverse**.

**Was ist ein Inverses?**
Das Inverse einer Zahl `a` ist die Zahl, mit der man `a` multiplizieren muss, damit genau `1` herauskommt.
Beispiel modulo 7: Was ist das Inverse von 3?
Wir suchen eine Zahl `x`, sodass `(3 · x) % 7 == 1`.
Probieren wir es aus:

- 3 · 1 = 3
- 3 · 2 = 6
- 3 · 3 = 9 ≡ 2 (mod 7)
- 3 · 4 = 12 ≡ 5 (mod 7)
- 3 · 5 = 15 ≡ 1 (mod 7)  -> Treffer!

Das modulare Inverse von 3 (modulo 7) ist also **5**. Statt "durch 3 zu teilen", multiplizieren wir in dieser Modulo-Welt einfach mit 5.

*Wichtig:* Ein Inverses existiert nur, wenn die Zahl und der Modulus (m) teilerfremd sind, ihr größter gemeinsamer Teiler (gcd / ggT) also 1 ist.

---

## 3) Gleichungen auflösen (Lineare Kongruenzen)

Schauen wir uns an, wie wir einfache Gleichungen nach `x` auflösen, wenn alles modulo `m` stattfindet.

### Addition: `(x + 9) mod 12 = 3`

Wir schreiben es in mathematischer Notation:
`x + 9 ≡ 3 (mod 12)`
Wir rechnen auf beiden Seiten minus 9:
`x ≡ 3 − 9 ≡ -6 (mod 12)`
Eine negative Zeit auf der Uhr (6 Stunden zurück) ist dasselbe wie +6.
`x = 6`

### Multiplikation: `(x · 6) mod 14 = 4`

Hier haben wir ein Problem: Wir wollen durch 6 teilen, aber 6 und 14 sind nicht teilerfremd (beide sind durch 2 teilbar).
**Lösungsweg:**

1. Teile die ganze Gleichung (inklusive Modulus!) durch den gemeinsamen Teiler (2).
2. Aus `6x ≡ 4 (mod 14)` wird `3x ≡ 2 (mod 7)`.
3. Jetzt suchen wir das Inverse von 3 modulo 7. Wie oben gezeigt, ist das 5.
4. Wir multiplizieren beide Seiten mit 5:
   `x ≡ 2 · 5 ≡ 10 (mod 7)`
5. `10 mod 7 = 3`. Unsere Basislösung ist `x = 3`.

---

## 4) Fortgeschrittenes Beispiel: Mehrere Primzahlen-Offsets finden

Stell dir vor, du hast ein System, bei dem drei Zyklen mit unterschiedlichen Längen (Primzahlen) laufen. Du suchst den genauen Punkt `x`, an dem alle drei Zyklen einen bestimmten „Offset“ (Rest) erreichen.
Dieses Problem löst der **Chinesische Restsatz (Chinese Remainder Theorem - CRT)**.

**Das Problem:**
Finde eine Zahl `x`, für die gilt:

- `x % 3 == 2`  (Rest 2 bei Division durch 3)
- `x % 5 == 3`  (Rest 3 bei Division durch 5)
- `x % 7 == 2`  (Rest 2 bei Division durch 7)

**Der Lösungsweg:**

1. **Gesamt-Modulus (M) berechnen:** Multipliziere alle Primzahlen.
   $$M = 3 \cdot 5 \cdot 7 = 105$$
   Unser gesuchtes `x` wird sich alle 105 Schritte wiederholen.

2. **Hilfszahlen ($M_i$) berechnen:** Teile M durch die jeweilige Primzahl.
   - $M_1 = 105 / 3 = 35$
   - $M_2 = 105 / 5 = 21$
   - $M_3 = 105 / 7 = 15$

3. **Inverse ($y_i$) für jede Hilfszahl finden:** Wir brauchen das Inverse von $M_i$ modulo der jeweiligen Primzahl.
   - Für 3: Was ist das Inverse von 35 modulo 3?
     (35 % 3 = 2. Das Inverse von 2 mod 3 ist 2, denn $2 \cdot 2 = 4 \equiv 1$). Also: **$y_1 = 2$**
   - Für 5: Was ist das Inverse von 21 modulo 5?
     (21 % 5 = 1. Das Inverse von 1 mod 5 ist 1). Also: **$y_2 = 1$**
   - Für 7: Was ist das Inverse von 15 modulo 7?
     (15 % 7 = 1. Das Inverse von 1 mod 7 ist 1). Also: **$y_3 = 1$**

4. **Alles zusammensetzen:** Die Formel lautet:
   $$x = (Rest_1 \cdot M_1 \cdot y_1 + Rest_2 \cdot M_2 \cdot y_2 + Rest_3 \cdot M_3 \cdot y_3) \pmod M$$

   Einsetzen unserer Werte:
   $$x = (2 \cdot 35 \cdot 2 + 3 \cdot 21 \cdot 1 + 2 \cdot 15 \cdot 1) \pmod{105}$$
   $$x = (140 + 63 + 30) \pmod{105}$$
   $$x = 233 \pmod{105}$$

   Wie oft passt die 105 in die 233? Zweimal (210). Es bleibt ein Rest von 23.
   **Lösung:** `x = 23`

   *(Probe: 23 % 3 = 2. 23 % 5 = 3. 23 % 7 = 2. Es stimmt!)*

---

## 5) Häufige Stolpersteine für Programmierer

- **Negative Reste:** In der Mathematik ist `-3 mod 12 = 9`. Viele Programmiersprachen (wie C, C++ oder Java) geben bei `-3 % 12` jedoch `-3` zurück. In Sprachen wie Rust solltest du für echte mathematische Modulo-Operationen immer `.rem_euclid(m)` verwenden anstelle des `%`-Operators.
- **Division durch Null:** Wenn du modulo `m` rechnest und das Inverse nicht existiert, verhält sich das so, als würdest du durch Null teilen. Immer vorher den `gcd` checken!

---

## 6) Beispiel aus der Praxis: Schnelles Potenzieren (modpow)

Oft muss man riesige Zahlen berechnen, wie `7^560 mod 561`. Wenn man zuerst `7^560` ausrechnet, sprengt das den Speicher jedes Computers. Die Lösung ist *Square-and-Multiply*: Wir berechnen die Potenz in kleinen Schritten und ziehen nach jedem Schritt sofort wieder Modulo.

### Implementierung in Rust

Für standard Datentypen nutzt man am besten einen kleinen Puffer (wie `u128`), um Overflow bei der Multiplikation zu vermeiden:

```rust
/// Berechnet (base^exp) % modulus sehr effizient.
pub fn modpow_u64(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    assert!(modulus != 0, "modpow: modulus darf nicht 0 sein");
    if modulus == 1 { return 0; }
    
    let mut result = 1;
    base = base % modulus; // Direkt am Anfang reduzieren
    
    while exp > 0 {
        // Wenn der aktuelle Exponent ungerade ist, aufs Ergebnis multiplizieren
        if (exp & 1) == 1 {
            result = ((result as u128 * base as u128) % (modulus as u128)) as u64;
        }
        // Basis für den nächsten Durchlauf quadrieren
        base = ((base as u128 * base as u128) % (modulus as u128)) as u64;
        // Exponent halbieren (Bit-Shift nach rechts)
        exp >>= 1;
    }
    result
}
```
