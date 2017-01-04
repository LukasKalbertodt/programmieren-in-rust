Aufgabe 2: Schnelles Implementieren von `fmt` Traits
====================================================

In [Aufgabe 6.3][a63] solltet ihr den fragwürdigen Typ `Swagger` implementieren.
Die konkrete Aufgabe war recht sinnlos, aber das Wrapper-Pattern allgemein ist sehr nützlich, wie wir kurz in der Vorlesung besprochen haben.

Beim letzten mal sollte jedoch nur `Display` für `Swagger<T: Display>` implementiert werden.
Allerdings gibt es ja deutlich mehr Traits zur Ausgabe von Werten, z.B. auch `Debug`.
In dieser Aufgabe sollt ihr [alle Formatting-Traits des `fmt` Moduls][std-fmt] (außer `Pointer`) für `Swagger` implementieren, wenn der innere Typ diese Traits implementiert.
Doch natürlich fällt euch auf, dass ein manuelles Vorgehen in ziemlich viel doppeltem oder ähnlichem Code enden würde.
Daher sollt ihr ein Makro schreiben, um ähnlichen/doppelten Code möglichst zu vermeiden!

Schreibt außerdem ein bisschen Beispielcode in `main()`, um die Implementationen zu auszuprobieren.



[a63]: https://github.com/LukasKalbertodt/programmieren-in-rust/tree/master/aufgaben/sheet6/task3
[std-fmt]: https://doc.rust-lang.org/std/fmt/index.html#formatting-traits
