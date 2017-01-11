Aufgabe 1: Assembly analysieren
===============================

In der Datei `magic.s` befindet sich der Assembly-Output einer geheimen Funktion.
Diese Funktion wurde mit `rustc 1.14` und aktivierten Optimierungen kompiliert.
Der Output wurde von mir nicht mehr verändert, entspricht also genau dem, was ihr auch sehen würdet, wenn ihr eure eigenen Programme analysieren wollt.

Analysiert dieses Assembly, um herauszufinden, was die Funktion tut. Als kleiner Hinweis: Die Funktion hat folgende Signatur:

```rust
fn magic(u64) -> bool
```

Bestückt dazu das Assembly mit ausreichend Kommentaren, in denen ihr beschreibt, was jede Instruktion tut und warum das dann zu dem gewünschten Ergebnis führt.

*Hinweise*:
- Beachtet die Slide "komische Instruktionen".
- Unbekannte Instruktionen kann man meist einfach mit "jae instruction" googlen.
    - Die meisten Seiten, die man findet bieten viel zu viele Informationen für Anfänger. Hier einfach mit einem filternden Blick die Beschreibungen für Menschen und die relevanten Informationen durchlesen.
- Ihr könnt euch am besten auf dem Papier immer aufschreiben, welcher Wert gerade in welchem Register ist. Sonst verwirren die Registernamen schnell.
