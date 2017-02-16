Aufgabe 3.1: Rule 90
====================

Zelluläre Automaten simulieren ein System von Zellen mit diskreten Werten in
diskreten Zeitschritten, wobei sich der Zustand des Systems zum Zeitpunkt
*n + 1* deterministisch aus dem Zustand zum Zeitpunkt *n* berechnen lässt.
In den meisten Fällen hängt der Wert einer Zelle allein von seinem eigenen Wert
und den Werten seiner Nachbarn zum vorherigen Zeitpunkt ab.

["Rule 90"][1] ist ein eindimensionaler, zellulärer Automat, dessen Zellen
boolesch sind, also nur zwei Werte annehmen können.
Oft spricht man bei einer Zelle mit einer 0 von einer "toten Zelle", bei einer
1 von einer "lebenden" Zelle.

Diese Zellen können nun nach bestimmten Vorschriften in diskreten Zeitschritten
simuliert werden. Der Wert einer Zelle zu dem Zeitschritt *n + 1* hängt von
seinem Wert und der Wert seiner beiden direkten Nachbarn zum Zeitpunkt *n* ab.
Sind z.B. zum Zeitpunkt *n* alle drei Zellen tot, wird die mittlere der Zellen
zum Zeitpunkt *n + 1* ebenfalls tot bleiben.

Die vollständigen Regeln lauten wie folgt:

| Vorheriger Zeitschritt        | `111` | `110` | `101` | `100` | `011` | `010` | `001` | `000` |
| ----------------------------- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- |
| Neuer Wert für mittlere Zelle | `_0_` | `_1_` | `_0_` | `_1_` | `_1_` | `_0_` | `_1_` | `_0_` |

Nun kann man die diskreten Zeitschritte zeilenweise anzeigen. Das sieht dann
z.B. so aus (in diesem Beispiel ist der Automat 5 Zellen groß):

```
0 1 1 0 0
1 1 1 1 0
1 0 0 1 0
0 1 1 0 0
```

Zu diesem Zeitpunkt sollte man sich fragen: Was ist mit den Randzellen?
In dieser Aufgabe gehen wir davon aus, dass der Anfang und das Ende des Systems
verbunden sind, d.h. der linke Nachbar der ersten Zelle ist die letzte Zelle
und der rechte Nachbar der letzten Zelle ist die erste Zelle.

---

In dieser Aufgabe sollt ihr ein Programm schreiben, welches den Automaten
"Rule 90" für Nutzereingaben simuliert. Diese Funktionalität soll in mehrere
Funktionen aufgeteilt werden.


### a) Eingabe einlesen

Zunächst muss vom Nutzer die initiale Konfiguration des Automaten eingelesen
werden.
Dafür soll eine Funktion `read_input()` geschrieben werden, die teilweise
bereits von uns bereitgestellt wurde.
In dem schon existierenden Code wird lediglich ein String vom Nutzer
eingelesen, der nur '0' und '1' enthält.
Eure Aufgabe ist es, aus diesem String einen `Vec<bool>` zu erstellen, der für
jede '0' im String einen `false`-Eintrag und für jede '1' einen `true`-Eintrag
enthält (in der richtigen Reihenfolge).
Dieser Vektor soll dann von der Funktion zurückgegeben werden.


### b) Zeitschritt simulieren

Nun sollt ihr eine Funktion `next_step()` schreiben, die aus der Konfiguration
des Automaten zu einem Zeitpunkt den nächsten Zeitpunkt berechnen soll.
Den Automaten zu einem Zeitpunkt stellen wir weiterhin einfach als ein
Boolean-Array dar; es muss also kein neuer Typ erstellt werden.


### c) Funktionen in `main()` benutzen

Nun müssen beide Funktionen noch genutzt werden.
In der `main()` Funktion wird eine initiale Konfiguration des Automaten
eingelesen, welcher dann über z.B. 20 Zeitschritte simuliert werden soll.

In jedem Zeitschritt soll der Zustand in einer Zeile auf dem Terminal
ausgegeben werden. Die Ausgabe kann entweder in Form von 0en und 1en geschehen
(wie oben im Beispiel) oder ihr könnt andere eindeutige Zeichen verwenden.
Es würde sich z.B. anbieten, für tote Zellen zwei Leerzeichen und für lebende
Zellen zwei U+2588 FULL BLOCK ('█') auszugeben:

```
  ████
████████
██    ██
  ████
```

Wenn alles klappt, probiert mal die initiale Konfiguration
`0000000000000001000000000000000` aus!


[1]: https://en.wikipedia.org/wiki/Rule_90
