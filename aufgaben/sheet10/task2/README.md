Aufgabe 2: Zufalls-Tools
========================

(*Diese Aufgabe hat nichts direkt mit den Low Level Themen in dieser Woche zu tun! Es geht nur darum, ein bisschen Rust zu programmieren.*)

In dieser Aufgabe soll ein Kommandozeilentool entstehen, welches diverse kleine Hilfsmittel rund um Zufall bereitstellt.
Es sollen drei Unterbefehle eingebaut werden:

- `coin` ("Wirft" Münze)
- `dice` ("Rollt" Würfel mit beliebig vielen Seiten)
- `choose` (Wählt aus einer Liste von Elementen mehrere Elemente aus)

Hier sind einige Beispiele der Benutzung (hier heißt das Programm `flip`):

```
$ flip coin
heads
$ flip dice
4
$ flip dice --sides=66
23
$ flip choose Ursula Peter Sabine
Peter
$ flip choose --count=2 Ursula Peter Sabine
["Sabine", "Peter"]
```

Wie zu sehen ist, lässt sich der `dice` Unterbefehl mit dem Parameter `--sides` anpassen.
Beim `choose` Unterbefehl kann durch den `--count` Befehl bestimmt werden, wie viele Elemente aus der Liste ausgewählt werden sollen (Achtung: Es sollen keine Elemente doppelt gewählt werden; zwei mal "Peter" wäre im obigen Beispiel ungültig).

Das ganze Programm lässt noch einen globalen Parameter zu: `--times`. Mit diesem kann man bestimmen, wie oft der angegebene Unterbefehl ausgeführt werden soll. Beispiel:

```
$ flip --times=3 coin
heads
heads
tails
$ flip --times=2 choose --count=2 Ursula Peter Sabine
["Peter", "Sabine"]
["Sabine", "Ursula"]
```

### Implementierung

Zum Kommandozeilenargumente-Parsing sollt ihr die [Crate `clap`](https://crates.io/crates/clap) nutzen.
Zum generieren von Zufallszahlen die [Crate `rand`](https://crates.io/crates/rand).
Die Aufgabe soll als Cargo-Projekt bearbeitet werden (ich hoffe jeder von euch hätte daraus sowieso ein Cargo-Projekt gemacht).


### Vorgehen

Geht bei dieser Aufgabe Schritt für Schritt vor. Kümmert euch erstmal um das Argument-Parsing mit `clap`. Clap ist recht gut dokumentiert, aber auch sehr umfangreich. Daher ist es einfach, ein bisschen den Überblick zu verlieren. Orientiert euch an den Beispielen und fragt ruhig nach, wenn ihr eine spezielle Funktion sucht, aber nicht findet. Es ist auch nicht tragisch, wenn ihr die Benutzung nicht ganz so hinbekommt wie oben gezeigt (grob sollte es sich aber schon daran orientieren).

Nachdem ihr erstmal nur das `--times` Argument geparst habt, kümmert euch dann um das Auswählen des Unterbefehls. Clap unterstützt sog. `SubCommands` direkt und hilft euch dabei also. Hinweis: Argumente können entweder global sein (wie `--times`) oder zu einem Subcommand gehören (wie `--sides`).

Überlegt euch auch eine sinnvolle Struktur für die unterschiedlichen Unterkommandos. Programmiert erstmal eine Dummy-Implementation dieser Kommandos, welche nur den Namen des Unterkommandos ausgibt, und stellt so sicher, dass das Auswählen des richtigen Unterkommandos funktioniert.

Wenn ihr also mit Clap zum größten Teil fertig seid, könnt ihr euch jetzt daran machen, die eigentlichen Unterbefehle zu implementieren. Hierzu braucht ihr natürlich dann die `rand` Crate. Deren Dokumentation ist auch nicht schlecht, aber auch diese Crate bietet einigermaßen viele Features (z.B. unterschiedlich RNGs), die einen schnell überfordern können. Auch hier macht es sicher Sinn, sich an den Beispielen zu orientieren.

Denkt daran, dass es sich durchaus lohnt, bereits nach Zwischenschritten mal zu comitten!


### Abgabezeitraum

Da diese Aufgabe eventuell mehr Zeit als üblich einnimmt, habt ihr die *Möglichkeit*, einen Teil dieser Aufgabe erst in der nächsten Woche nachzureichen. In dieser Woche muss eure Applikation schon folgendes können:

- Den Unterbefehl richtig auswählen
- `coin` soll schon funktionieren
- `dice` und `choose` sollen ihre Namen printen
- `--times` soll auch schon funktionieren (der Unterbefehl wird dann mehrmals ausgeführt)

Falls ihr es in dieser Woche nicht mehr schafft, *könnt* ihr die folgenden Aufgaben erst in der nächsten Woche einreichen:

- `dice` Implementation
- `choose` Implementation

Es sei euch aber ans Herz gelegt, die komplette Aufgabe in dieser Woche zu bearbeiten, wenn möglich.


### Testen

Wie immer könnt ihr eure Applikation mit `cargo run` ausführen. Um die Argumente an eure Funktion zu übergeben, verwendet zwei einzeln stehende `--`. Z.B. so:

```bash
$ cargo run -- dice --sides=4
#              \------------/
#               for your app
```

Eure Anwendungen könnt ihr auch mit `cargo install` installieren, sodass ihr sie überall einfach mit dem Namen aufrufen könnt (wie oben `flip`).


### Weitere Hinweise

Achtet darauf, dass der Code, den ihr einreicht, idiomatischer Rust Code ist. Überlegt euch z.B., was euer Tutor bislang immer an euren PRs zu kommentieren hatte und versucht das direkt zu vermeiden. Doppelter Code sollte natürlich auch in dieser Aufgabe vermieden werden.
