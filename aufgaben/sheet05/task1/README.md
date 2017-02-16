Aufgabe 5.1: Pokemon umstrukturieren und bunt machen
====================================================

a) Umstrukturieren
------------------

In der ersten Teilaufgabe geht es darum, unser ziemlich großes Pokemon-Spiel
auf mehrere Module aufzuteilen.
Außerdem soll dieses Spiel nun ein Cargo-Projekt sein.
Fangt am besten mit einem leeren Cargo-Projekt an, baut dann die grundlegende
Modulstruktur und kopiert dann Codeteile aus der Lösung zur vorherigen Aufgabe
in die passenden Module.
Ihr könnt entweder eure Lösung oder [die Musterlösung][ml] nutzen.

Das Projekt soll folgendermaßen strukturiert werden:

- *Wurzel*: Hier befindet sich nur `main()`, die auch selber nur wenig
  Code enthält.
- `db`: enthält Funktion `pokemon_by_name()`.
    - `data`: enthält die konstanten Arrays.
    - `types`: enthält viele wichtige Typen (allerdings ohne `impl` Block,
      also nur Datenlayout):
      `Attack`, `AttackCategory`, `TypeEffectiveness`, `Type`, `PokemonType`,
      `PokemonModel`, `Stats`.
- `engine`: enthält den Typen `Pokemon` mit Methoden.
    - `canon`: Enthält alle Funktionen und Methoden die Formeln/Algorithmen aus
      dem Pokemon-Universum abbilden.
- `game`: Enthält alle Funktionen, die irgendetwas einlesen oder ausgeben.
  Insb. soll hier eine Funktion `fight()` definiert werden, die den ganzen
  Kampf abwickelt (damit `main()` schön klein bleibt).

Ihr werdet merken, dass ihr nach dem Einfügen des alten Codes viele Pfade
und andere Dinge ändern müsst.
Benutzt bei `use`-Anweisungen keine Wildcards; die einzige Ausnahme ist das
`types` Modul, aus welchem alle Namen einfach mit einem Wildcard genutzt werden
*können*.

Nach einiger Zeit sollte der Compiler dann endlich zufrieden sein und das
Spiel sollte genau so wie vorher laufen.


[ml]: https://github.com/LukasKalbertodt/programmieren-in-rust/tree/master/aufgaben/sheet3/sol2


b) Farben einbauen
------------------

Um einmal den Umgang mit externen Crates zu üben, sollt ihr in der zweiten
Teilaufgabe die Ausgabe eures Pokemon-Spiels etwas bunter gestalten.
Terminals und dergleichen unterstützen oft irgendeine Art von farblichem Text,
oft allerdings mit nur 8 unterschiedliche Farben.
Um farbigen Text auszugeben, müsste man allerdings Code schreiben, der
plattformabhängig und auch oft unschön ist.
Das wollen wir natürlich vermeiden!

Aber selbstverständlich hatte schon mal jemand das Problem.
Daher gibt es für unseren Zweck schon diverse Libraries, die wir nutzen können.
Wie eine schnelle Suche auf [crates.io](https://crates.io) zeigt, gibt es
folgende Möglichkeiten:

- [`term`](https://crates.io/crates/term): Platform**un**abhängige Library,
  allerdings recht *low-level*, also aufwändiger zu bedienen.
- [`ansi_term`](https://crates.io/crates/ansi_term): Plattform**abhängige**
  Library, die nur für Ansi-Terminals funktioniert.
- [`term-painter`](https://crates.io/crates/term-painter): Diese Library ist
  von mir. Sie baut auf `term` auf, ist also plattform**un**abhängig, aber ist
  wesentlich bequemer zu bedienen. Es klingt natürlich direkt so, als würde ich
  Werbung machen, aber ich denke, dass es tatsächlich die beste Wahl ist, wenn
  man plattformunabhängig programmieren möchte.
- ... und ein paar andere

Eine Library davon sollt ihr nutzen, um eure Ausgabe bunt zu gestalten.
Ihr könnt z.B. Pokemon-Namen immer in einer bestimmten Farbe einfärben und
Attacken immer in einer anderen.
Hauptsache die Ausgabe ist ein wenig übersichtlicher und bunt.

Das Hauptproblem bei dieser Aufgabe wird wahrscheinlich sein, die Dokumentation
von fremden Libraries zu lesen und zu verstehen.
Ich denke aber, dass oft genug Beispiele bereitstehen, um zu verstehen, was man
machen muss.
Sonst gibt es ja immer noch Piazza ;-)
