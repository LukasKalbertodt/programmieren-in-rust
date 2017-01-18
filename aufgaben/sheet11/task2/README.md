Aufgabe 2: Tic Tac Toe
======================

In dieser Aufgabe sollt ihr das Spiel *Tic Tac Toe* implementieren.
Dabei soll es unterschiedliche Spieler geben: Eine dumme KI, eine schlauere KI und natürlich einen menschlichen Player, der seine Züge im Terminal eingibt.

Es geht in dieser Aufgabe einfach wieder darum, Rust-Code zu schreiben. Ohne Programmiererfahrung wird man nicht besser und mit hochspezialisierten Aufgaben sammelt man eher wenig Erfahrung. Es dreht sich hier also nicht nur speziell um ein Thema, sondern um alles bereits Gelernte.

Ein bisschen besonders an dieser Aufgabe ist, dass es nicht viele Vorgaben gibt. *Wie* ihr das Spiel implementiert, ist euch überlassen. Allerdings solltet ihr den Code selbstverständlich möglichst idiomatisch halten. Auch darum geht es in dieser Aufgabe: Ein eigenes, mittel-großes Projekt zu entwerfen und verschiedene Programmstrukturen auszuprobieren und gegeneinander abzuwägen.

- Überlegt euch eine gute Modulstruktur. In Rust gibt es keine feste Regel, wie man seine Module aufteilt, also muss man hier selber nachdenken. Stopft nicht alles in ein Modul/eine Datei sondern überlegt euch, welche Struktur Sinn ergibt.
- Identifiziert doppelten Code im ganzen Programm. Doppelter Code sollte *immer* vermieden werden.
- Überlegt euch für jeden Codeabschnitt, ob dieser idiomatisch und maximal knapp formuliert ist. Kann man es mit einem Iterator kürzer schreiben? Kann man mit einer tollen Helferfunktion von `Option` eine Zeile abkürzen? Dann tut das!
- Achtet darauf, das starke Typsystem zu nutzen. Ihr könntet auch alle eure Variablen als `String`s speichern; nur leider wird euch das Typsystem in dem Fall nicht helfen. Genau so könnt ihr für alle Zahlen `i32` benutzen (Hallo Java!), aber das ist in vielen Fällen auch nicht sinnvoll. Habt ihr einen Datentypen der nur die Werte von 3 bis 11 annehmen kann? Dann erstellt euch einen neuen Typen dafür, eventuell mit Hilfsfunktionen. Habt ihr einen Typen der nur sehr wenige, nicht numerische Möglichkeiten repräsentieren soll? Nutzt ein `enum`!

An solche Fragen muss sich ein Gehirn gewöhnen, bis man schließlich schnell guten Code schreiben kann.


Anforderungen an das Spiel
--------------------------

Was soll es denn nun können? Beim Aufruf kann der Benutzer zwei Spieler angeben, z.B.:

```
$ ttt human smart-ai
```

Ihr könnt diese Angaben aber auch per stdin einlesen. Falls ihr es als Kommandozeilenargumente einlest, könnt ihr wieder "clap" benutzen, eine andere Crate zum parsen von Parametern oder auch einfach `std::env`-Funktionen.

Es soll drei unterschiedliche Spieler geben:

- Der "Mensch" Spieler liest den nächsten Zug von stdin ein (wo ein echter Mensch etwas eingeben muss). Die Eingabe kann in einem beliebigen Format geschehen, ich habe z.B. eine Schach-ähnliche Notation `"b3"` benutzt.
- Die dumme künstliche Intelligenz wählt immer irgendeinen gültigen Spielzug aus, wobei gültig nur heißt, dass das gewählte Feld noch nicht besetzt ist.
- Die "schlaue" künstliche Intelligenz könnt ihr so schlau machen, wie ihr wollt, aber sie soll mindestens "Siegerzüge" erkennen und ausführen. D.h. also wenn die KI im aktuellen Zug gewinnen könnte, soll sie diesen auch auf jeden Fall ausführen.

Beide Spieler werden also nun immer abwechselnd aufgefordert, einen Zug auszuführen. Sobald ein Spieler gewonnen hat, soll der Gewinner ausgegeben und das Spiel beendet werden. Nützlich wäre auch, wenn nach jedem Zug das Feld einmal auf dem Terminal ausgegeben wird.


Zeit
----

Da diese Aufgabe eventuell mehr Zeit einnehmen könnte, dürft ihr wieder einen Teil der Aufgabe in der nächsten Woche erledigen (eine komplette Lösung in dieser Woche ist natürlich trotzdem besser).

Diese Woche soll insb. schon funktionieren:
- Auswählen vom Spieler per Terminal
- Abwechselnde Züge beider Spieler
- Anpassen des Spielfeldes nach jedem Zug
- Menschlicher Spieler
- Dumme künstliche Intelligenz

Diese Woche muss noch nicht *unbedingt* funktionieren:
- Erkennung eines Sieges (beendet einfach wenn alle Felder voll sind)
- "Schlaue" künstliche Intelligenz
