Aufgabe 2: Sortieren
====================

In dieser Aufgabe sollt ihr Funktionen schreiben, die eine Slice in-place sortieren.
Einige davon erlauben es dem Nutzer, anders zu sortieren, als von `Ord` (also der natürlichen Sortierung) vorgegeben wird.
Die Funktionen sollen wieder so allgemein wie möglich formuliert werden und Code-Doppelung soll möglichst vermieden werden.

Ihr findet schon ein Programmgerüst mit Tests in `sort.rs`.

Programmiert werden sollen folgende drei Funktionen:

1. `sort_by()`: Bekommt die mutable Slice und ein Funktionsding, welches zu zwei Elementen aus der Slice (als Referenz übergeben) ein `std::cmp::Ordering` zurückgibt. Anhand diesem Comparator soll dann sortiert werden. Die natürliche Sortierung kann man z.B. mit `|a, b| a.cmp(b)` erreichen (falls der Elementtyp `Ord` implementiert). Andersherum sortieren geht z.B. mit `|a, b| b.cmp(a)`.

2. `sort()`: Bekommt nur die mutable Slice und sortiert diese nach der natürlichen Sortierung.

3. `sort_by_key()`: Bekommt die mutable Slice und ein Funktionsding, welches eine Referenz auf ein Element im Array auf einen anderen Wert map't. Ein Integer-Array kann man also mit `|x: &i64| x.abs()` nach dem Betrag der Werte sortieren.

Weitere Beispiele gibt es in den Unittests.

Den Sortieralgorithmus dürft ihr komplett frei wählen.
Wenn man mehr freie Zeit über die Feiertage möchte, sollte man also etwas recht einfaches, wie Selection Sort, nutzen ;-)
