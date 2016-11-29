Aufgabe 2: Vektor
=================

In dieser Aufgabe soll ein einfacher, generischer Vektor-Typ erstellt werden.
Wieder soll eine Library-Crate als Cargo-Projekt angelegt werden und im `tests`
Untermodul sollen ein paar kurze Unittests die eigentliche Funktionalität
testen.

Der Datentyp soll `Vector2` heißen und besteht aus einer x- und y-Koordinate.
Der Typ dieser Koordinaten ist jedoch generisch und kann grundsätzlich erstmal
alles sein. Neben den expliziten Anforderungen in a) und b) kann und/oder
muss `Vector2` noch weitere Traits implementieren, solange das möglich ist.


### a) Konstruktor-Funktionen

Wir möchten vier Konstruktor-Funktionen zu `Vector2` hinzufügen:

- `new(x, y)`: bekommt beide Koordinaten als Parameter
- `origin()`: Vektor am Nullpunkt (beide Koordinaten 0)
- `unit_x()`: Ein Einheitsvektor mit x=1
- `unit_y()`: Ein Einheitsvektor mit y=1

Um diese Funktionen implementieren zu können, greifen wir auf schon vorhandene
Traits zu. Diese finden wir in der Crate `num-traits`, die es wie gewohnt auf
crates.io gibt. Von dieser externen Bibliothek müsst ihr euch passende Traits,
die ihr nutzen wollt/müsst, aussuchen (wir sind nur an den Traits der
Bibliothek interessiert!).


### b) Operatoren überladen

Für den Vektor soll der `+` und `*` Operator überladen werden (wenn dies
möglich ist).
Dabei ist das eine natürlich die Vektoraddition (also Vektor + Vektor), das
andere ist die Skalarmultiplikation (also Vektor * Skalar).
