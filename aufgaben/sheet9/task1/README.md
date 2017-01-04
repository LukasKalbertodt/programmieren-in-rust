Aufgabe 1: `try!` für `Option`
==============================

Wir hätten gerne ein Makro `try_opt!()`, welches wie `try!()` funktioniert, allerdings für `Option`s.
`try_opt!(Some(3))` soll zu "3" evaluieren, während `try_opt!(None)` ein early return auslösen soll.

Die beiden vorgegebenen Funktionen sollen wie erwartet funktionieren.
