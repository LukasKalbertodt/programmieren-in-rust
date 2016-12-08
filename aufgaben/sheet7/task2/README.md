Aufgabe 2: Diverse Funktionen
=============================

In dieser Aufgabe sollen diverse kleinere Funktionen enstehen.
In der Musterlösung habe ich immer irgendwie Iteratoren und Iterator-Methoden genutzt, also liegt es nahe, dass ihr es auch tun solltet.
Wenn ihr natürlich meint, eine schönere Lösung ohne Iteratoren gefunden zu haben, könnt ihr sie eurem Tutor natürlich auch vorstellen ;-)

Versucht bei dieser Aufgabe alle Funktionen immer möglichst kurz (also mit wenigen Anweisungen) zu implementieren. Ihr müsst also auch nicht so sehr auf die Laufzeit eurer Algorithmen achten.

Euch ist ein Gerüst mit diversen Unittests gegeben. Die Testfälle erklären die Funktionen zusätzlich.

*Hinweis*: Es könnten für diese Aufgabe auch Funktionen direkt aus dem Modul `std::iter` sinnvoll sein.

### Funktionen

- `factorial()`: Berechnet `x!` für ein gegebenes `x`.

- `rot13()`: "Verschlüsselt" einen gegebenen String mit der derzeit besten [Verschlüsselungsmethode Rot13](https://de.wikipedia.org/wiki/ROT13). Die einzige noch bessere Verschlüsselung ist Rot26. Falls ihr meint, dass ihr es hinbekommt, könnt ihr zusätzlich auch gerne Rot26 implementieren...

- `is_palindrome()`: Testet ob ein gegebener String ein Palindrom ist.

- `used_chars_count()`: Bekommt eine Slice von String-Slices übergeben und gibt die Anzahl der unterschiedlichen Zeichen in allen Strings zurück. Whitespace-Zeichen sollen aber nicht mitgezählt werden. Schaut euch hier den Unittest für Beispiele an. Ihr könnt hier auch gerne eine passende collection aus `std` benutzen... 

- `greatest_subsequencial_sum()`: Bekommt ein Array von ganzen Zahlen gegeben und sucht nun das Subarray (die Slice), bei dem die Summe aller Zahlen am größten unter allen möglichen Subarrays ist. *Tipp*: [Slices](https://doc.rust-lang.org/std/primitive.slice.html) definieren eine für diese Aufgabe *sehr* hilfreiche Methode. 
