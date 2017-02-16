Aufgabe 4: Kopierbefehl
=======================

(*In dieser Aufgabe geht es nicht mehr unbedingt nur um Iteratoren!*)

In dieser Aufgabe sollt ihr den `cp` Befehl nachprogrammieren ... Ok, nur einen Teil davon.
Euer Programm soll zwei Dateinamen als *command line parameter* erhalten und dann von der einen Datei in die andere kopieren:

```
$ mycp Cargo.toml Peter.toml
```

Euer Programm soll also `mycp` heißen!
Wenn ihr es richtig gemacht habt, könnt ihr mit dem Befehl `cargo install` euer Programm global auf eurem System installieren (die Executable wird in `~/.cargo/bin` kopiert, welches im `$PATH` ist). Damit ihr es aber nicht immer neu installieren müsst, um es auszuprobieren, könnt ihr es aber natürlich mit `cargo run` testen. Die Kommandozeilenparameter werden nach den `--` übergeben:

```
$ cargo run -- Cargo.toml Peter.toml
```

Die Hauptschritte zur Implementation sind folgende:

### a) Parsen der Kommandozeilenparameter

Diese müssen zunächst eingelesen werden und auf Gültigkeit überprüft werden (richtige Anzahl?).
Dies soll in einer eigenen Funktion geschehen, in der *keine* Ausgabe auf dem Terminal geschehen soll.
Alle Informationen müssen im Rückgabewert kodiert sein.

Hierzu ist sicherlich das [`std::env` Modul](https://doc.rust-lang.org/std/env/index.html) interessant.


### b) Kopieren des Dateiinhalts

Falls die Parameter korrekt waren, muss jetzt von einer Datei in die andere kopiert werden.
Das soll ebenfalls in einer eigenen Funktion passieren, welche *keine* Ausgaben auf dem Terminal macht.

Dabei sollten natürlich auch Fehler behandelt werden.
Das heißt aber nicht, dass ihr selber viel Code programmieren müsst; wahrscheinlich ist die Implementierung kürzer, als ihr anfangs vermuten würdet.

Wie man schon vermutet, wird hier das [`std::io` Modul](https://doc.rust-lang.org/std/io/index.html) eine Rolle spielen.
Nur die Funktion `std::fs::copy()` dürft ihr **nicht** nutzen! Das wäre ja zu einfach ;-)
