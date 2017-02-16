Aufgaben 4.2: Taschenrechner
============================

In dieser Aufgabe sollt ihr ein Programm schreiben, welches einfache
Rechenaufgaben lösen kann.
Zwar kann man sich einen sehr einfachen Rechner schnell manuell
zusammenschreiben.
Das Programm dieser Aufgabe soll jedoch nicht quick'n'dirty geschrieben werden,
sondern später einfach zu erweitern sein.

Ein Teil des Codes ist bereits in `calculator.rs` vorgegeben.
Das gegebene Programm liest in einer Endlosschleife Eingaben vom Nutzer ein,
tut damit aber bislang noch nichts, außer sie auszugeben.
Das Programm kann man bekanntlich mit Strg+C beenden.

Das Programm soll später ungefähr folgende Aufgaben lösen können:

```
calc > 3 + 3
6
calc > 2-3
-1
calc > 9 + (7 - 2)
14
calc > 2
2
calc > (3 - 2) + ((3 - 2) + 9)
11
```


a) Tokenization
---------------

Die ersten beiden Schritte fast aller Compiler jeglicher Art sind:
*Tokenization* (auch *Lexing*) und *Parsing*.
Gerade Compiler haben einen extrem komplizierten und komplexen Job.
Um dieser Komplexität Herr zu werden, werden eine Reihe von unterschiedlichen
Abstraktionsstufen genutzt.
Der Quellcode wird permanent von einer Darstellung in die nächste überführt;
Analysen, wie Typechecking ("werden alle Typen korrekt benutzt?"), werden nie
direkt auf dem Quellcode-String, sondern auf abstrakteren Darstellungen
ausgeführt.

Glücklicherweise müsst ihr keinen Compiler schreiben, sondern nur einen
Taschenrechner.
Aber auch dabei müssen Nutzereingaben mit einer bestimmten Syntax analysiert
werden.
Daher werden wir die Funktionalität des Rechners ebenfalls in drei Schritte
unterteilen:

1. Tokenization/Lexing
2. Parsing
3. Errechnen des Ergebnisses

Im ersten Schritt wird der Eingabestring in eine Liste von sog. *Tokens*
umgewandelt. Ein Token kann ein Zeichen repräsentieren (wie z.B. `"+"`) oder
auch mehrere (wie z.B. `"127"`).
Anstatt also eine Liste von Zeichen zu betrachten, betrachtet man eine Liste
von atomaren Elementen im Code.
Um mal ein Beispiel zu geben:

```
Eingabe:   "23        +        7"

             |        |        |
Tokens: [Number(23), Plus, Number(7)]
```

Eure erste Aufgabe ist es, einen Typen `Token `zu definieren, der einen Token
darstellt.
Der Tokenization-Schritt produziert nämlich dann `Vec<Token>`.
Wie oben schon gezeigt, muss der Rechner in der Lage sein, Addition und
Subtraktion von beliebigen positiven Ganzzahlen auszuführen (das Ergebnis kann
negativ sein, nur die einzelnen Zahlen in der Eingabe nicht).
Außerdem müssen Klammerausdrücke behandelt werden.
Entwerft entsprechend den `Token` Typ.

---

Nachdem der Typ definiert ist, müsst ihr jetzt eine Funktion `tokenize()`
schreiben.
Diese Funktion bekommt einen String und erzeugt daraus eine Liste von Tokens.
Natürlich können schon bei der Tokenization Fehler im Input auffallen.
Diese Fehler sollten nicht einfach ignoriert werden.

Die Funktion soll außerdem Whitespaces in der Eingabe komplett ignorieren, dafür
also keinen Token erstellen (das würde in späteren Schritten nur ablenken).

*Wichtig*: Ihr müsst lediglich mit Zahlen umgehen können, die nur
*eine* Ziffer besitzen.
Zahlen bestehend aus mehreren Ziffern zu lexen, ist -- selbst mit Hilfe von
der `parse()` Funktion aus der Standardbibliothek -- nicht ganz trivial.
Wer das trotzdem schafft, wird natürlich vom Tutor gelobt ;-)


Schließlich soll die Nutzereingabe an `tokenize()` gegeben werden und das
Ergebnis zu Debugzwecken in `main()` ausgegeben werden.
Folgende Eingaben können getestet werden:

- `3`
- `3 +       4`
- `3 - (7 - 7)`
- `3 + ) - (` -> (in diesem Schritt führt das *noch nicht* zum Fehler!)
- `peter` -> Hier sollte ein Fehler ausgegeben werden (nicht panic!)

**Tipps**:

- Es lohnt sich, die Dokumentation von
  [`char`](https://doc.rust-lang.org/std/primitive.char.html) durchzulesen
- Um Instanzen selbstdefinierter Typen miteinander vergleichen zu können, kann
  man `#[derive(PartialEq)]` über die Typdefinition schreiben. Dann kann der
  Typ per `==` und `!=` verglichen werden.

b) Definition des Parsetrees
----------------------------

Bisher haben wir aus einem String einen Tokenstream gemacht.
Im Parsing-Schritt wird aus dem Tokenstream ein sog. Parsetree erzeugt.
Doch bevor wir uns an das Parsing wagen, müssen wir erstmal Typen definieren,
um den Parsetree darzustellen.

Hier ist mal ein Beispiel eines Parsetrees zu der Eingabe `3 + (6 - 1)` zu
sehen:

```
            +---------+
            |  Summe  |
            +---------+
          /            \
         /              \
     +-----+        +-----------+
     |  3  |        | Differenz |
     +-----+        +-----------+
                    /          \
                   /            \
               +-----+        +-----+
               |  6  |        |  1  |
               +-----+        +-----+
```

Wir sehen also:

- Blätter des Baumes stellen die Literale/eingegebenen Zahlen dar
- Innere Knoten des Baumes wissen, welche Art von Rechenoperation sie
  darstellen und welche Kinder sie besitzen

Erstellt zunächst einen Typ `Op`, welcher eine Rechenoperation darstellt.
Dieser Typ soll dann eine Methode `apply()` bekommen.
Diese Methode bekommt zwei Zahlen und gibt das Ergebnis der Operation
angewendet auf diese beiden Zahlen zurück.

Danach soll der Typ `Expr` erstellt werden, der letztendlich den Parsetree
darstellt.
*Tipp*: Auch wenn wir immer nur zwei Kinder pro Knoten erwarten, könnt ihr hier
gerne `Vec<_>` benutzen!

Dieser Typ soll eine Methode `evaluate()` bekommen, welche den ganzen Baum
auswertet (also das Ergebnis ausrechnet).
Die Methode würde also auf dem oben gezeigten Baum "8" zurückgeben.
Das ist also Schritt 3 aus der oben genannten Liste von Schritten.

Baut euch in einer kleinen Hilfsfunktion manuell eine Instanz (z.B. den oben
gezeigten Baum) von `Expr` zusammen und testet `evaluate()` auf dieser Instanz.
Ruft diese Hilfsfunktion in `main()` auf, um das Ergebnis zu prüfen.
Löscht diese Hilfsfunktion auch nicht, sodass euer Tutor sie später nutzen
kann.


c) Parsing
----------

Dieser zweite Schritt ist der komplizierteste.
Ihr müsst diese Teilaufgabe nicht komplett lösen, da die komplette Lösung
nicht gerade trivial ist.
Mindestanforderung ist jedoch, dass ihr die folgenden Eingaben parsen könnt:

- `3`
- `1 + 2`
- `9 - 6`

Das Parsing soll in einer Konstruktorfunktion von `Expr` geschehen.
Nennen wir diese Funktion einfach `parse()`.
Die Funktion bekommt natürlich den vorher generierten Tokenstream und soll
bei Erfolg eine `Expr`-Instanz zurückgeben, welche den Parsetree darstellt.

Aber natürlich kann das Parsing auch auf Fehler in der Eingabe stoßen.
Zum Beispiel sollten Fehler wie eine leere Eingabe und eine falsche Syntax
gemeldet werden.

Ein Hinweis zur Syntax: Wir gehen davon aus, dass jeder Knoten im Baum nur zwei
Kinder hat und Rechnungen immer geklammert sind.
Eine Eingabe von `3 + 4 + 5` ist also ungültig und müsste als `3 + (4 + 5)`
oder `(3 + 4) + 5` eingegeben werden.
Das vereinfacht das Parsing stark.
Unsere Grammatik ist also:

```
expr    := ⟨operand⟩ [⟨op⟩ ⟨operand⟩]
operand := ⟨num⟩ | "(" ⟨expr⟩ ")"
op      := "+" | "-"
num     := "0" | "1" | ... | "9"
```

*Tipps*:

- Wenn ihr versucht, den kompletten Parser zu implementieren, bietet
  sich natürlich eine Art von Rekursion an.
- Es gibt recht viele Wege, diesen Parser zu implementieren. Ein Weg, den ich
  recht bequem finde: Man speichert sich den aktuellen Index im Tokenstream
  und erhöht diesen immer, wenn man weitere Tokens verarbeitet hat.
- Kleine Hilfsfunktionen können doppelten Code vermeiden.


---

Zuletzt (unabhängig davon, ob ihr nur den halben oder ganzen Parser
implementiert habt) soll alles in `main()` benutzt werden, um den
Taschenrechner funktionsfähig zu machen.
