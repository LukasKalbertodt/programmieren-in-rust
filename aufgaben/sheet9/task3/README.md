Aufgabe 3: Einfach Hashmap erstellen
====================================

Wir kennen ja das `vec![]` Makro, welches sich wunderbar eignet, um direkt einen Vektor mit bestimmten Werten zu erstellen.
Da es ein solches Makro nicht in der Standardbibliothek gibt, sollt ihr in dieser Aufgabe ein solches Makro schreiben.
Die genaue Syntax ist nicht so wichtig, aber ich könnt z.B. eine Ruby-ähnliche Syntax nutzen:

```rust
let ages = hash_map!{ "Sabine" => 26, "Peter" => 32 };
println!("{:#?}", ages);
```
