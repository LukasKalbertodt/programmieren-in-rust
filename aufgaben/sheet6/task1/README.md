Aufgabe 1: Hilfsfunktionen
==========================

In dieser Aufgabe soll eine Library-Crate (Cargo Projekt) erstellt werden,
welche ein paar kleine, aber nützliche Funktionalitäten bereitstellt.

Für alle drei Funktionen/Methoden soll ein Unittest geschrieben werden. Alle
Unittests liegen in dem `tests` Untermodul der Crate; alles andere kann aber
im Wurzelmodul definiert sein. *Hinweis*: Mit `cargo test` können alle
Unittests ausgeführt werden.


### a) `clamp()`

Diese Funktion nimmt drei Parameter: einen Wert sowie `min` und `max`.
Die Funktion returned einfach den gegebenen Wert, solange er zwischen `min` und
`max` liegt. Falls der gegebene Wert jedoch kleiner als `min` ist, wird `min`
returned; das gleiche entsprechend mit `max`.

Die Funktion soll so allgemein wie möglich formuliert werden und keine
genauen Typen festlegen.


### b) Summe und Produkt

Die Funktion nimmt zwei Argumente and und liefert sowohl die Summe als auch
das Produkt dieser beiden Werte zurück. Auch diese Funktion soll möglichst
allgemein formuliert werden.


### c) Extension Trait

Wir würden gerne `bool`-Variablen sehr einfach in ein `Option<T>` umwandeln
können, also z.B. so einen Code aufrufen können:

```rust
true.into_option(3);        // Some(3)
false.into_option("susi");  // None
```

Fügt die Methode zu `bool` mit Hilfe eines Extension Traits hinzu.
