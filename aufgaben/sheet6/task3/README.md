Aufgabe 3: Swagger
==================

*Diese Aufgabe kann in einer einzelnen `.rs` Datei gelöst werden und benötigt
kein Cargo-Projekt.*

Erstellt einen Typ `Swagger`, welcher einen beliebigen anderen Typen in sich
speichert.
`Swagger` soll mit `println!()` und dem normalen `{}` Platzhalter
ausgegeben werden können, wenn immer das möglich ist.
Die Ausgabe soll "yolo ??? swag" sein, wobei "???" für die Ausgabe des inneren
Objekts steht.
Wäre also eine `3` in einer Swagger-Instanz gespeichert und man würde diese
Instanz ausgeben, wäre die Ausgabe "yolo 3 swag".

Weiterhin soll es möglich sein, auf jedem beliebigen Typen direkt eine Methode
`with_swag()` aufzurufen, die diesen Typen in eine `Swagger`-Instanz verpackt.

```
println!("{}", 3.with_swag());    // prints: "yolo 3 swag"
```
