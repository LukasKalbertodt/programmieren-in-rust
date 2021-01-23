# Vorlesung „Programmieren in Rust“

Deutsche Vorlesung über Rust an der Universität Osnabrück, Wintersemester 2016/17. In diesem Repository werden Slides und andere Materialien zur Vorlesung gesammelt. Die **Ergebnisse der Evaluation** der Veranstaltung durch die Studenten sind [**hier**](https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/AuswertungEvaluation.pdf) einzusehen.

> **Hinweis**: Inzwischen sind die Inhalte in diesem Repository leider etwas veraltet. Die Vorlesung und Aufgaben können immer noch gut genutzt werden, um Rust zu lernen und zu verstehen, allerdings wird nicht überall der modernste Rust Code gezeigt. Themen wie `async`, Edition 2018 und proc-macros werden leider gar nicht behandelt. Dieses Repository sollte also nicht als alleinige Quelle zum Rust-Lernen genutzt werden. Leider werde ich auch die Folien nicht mehr aktualisieren, da dies zu viel Aufwand für recht wenig Nutzen ist. 

## Abschlussprojekte

Eine Liste aller Abschlussprojekte befindet sich [hier (Englisch)](https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/projects.md).

## Vorlesungsaufzeichnungen

Die Aufzeichnungen gibt es oft in zwei Versionen:

- nur das Beamerbild, aber in hoher 1080p-Qualität, mit der Aufnahme vom Notebook-Mikrofon ([YouTube Playlist](https://www.youtube.com/playlist?list=PL0Ur-09iGhpwMbNiVTBeHmIjs0GuIXhNg))
- Beamerbild und Dozent (mit Tafel) und Umhängemikrofon (Aufnahme schlägt hin und wieder fehl)

| Datum  | Thema | Nur Beamer | Dozent und Beamer |
| ------ | ----- | ------------------------ | ---------------------------- |
| 24.10. | [Warum Rust?/Organisatorisches][s0], [Grundlagen][s1] | [YouTube](https://www.youtube.com/watch?v=lQ36K1htRDY) | - |
| 26.10. | [Git und GitHub/Rust einrichten][s2], [Grundlagen][s1] | [YouTube](https://www.youtube.com/watch?v=k6KKO7QfhNQ) | - |
| 31.10. | [Grundlagen](s1), [Ownership-System (Teil 1)][s3], [Strings][s4] | [YouTube](https://www.youtube.com/watch?v=1dr2CDxBRuo) | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?id=2a7b1a55-5b47-4e13-bd11-45d5b6e3c2a2) |
| 02.11. | [Strings][s4], [Git (Teil 2)][s5] | [YouTube](https://www.youtube.com/watch?v=Hiez8zq3yNg) | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?id=15898062-d625-48f3-9bd4-518710271568) |
| 07.11. | [Structs, impl-Block und Methodensyntax][s6], [Diverses (Vec)][s7] | [YouTube](https://www.youtube.com/watch?v=EGogoHQUeLE) | - |
| 09.11. | [Diverses (Konstanten, Tuple-Struct, Match, Pattern)][s7], [Enums](s8) | [YouTube](https://www.youtube.com/watch?v=PHEYNPtWIbs) | - |
| 14.11. | [*Nachbesprechung Aufgaben*][t3], [Error Handling][s9] | [YouTube](https://www.youtube.com/watch?v=YYb0fIELE1Q) | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?id=2efa047b-8c2a-4545-bd6b-3c36d208f33f) |
| 16.11. | [Error Handling][s9], [*`tree.rs`-Beispiel*][m1] | [YouTube](https://youtu.be/lJzYi5TqtEY) | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?id=1cac1644-d054-4fce-b59b-6214730abf19) |
| 21.11. | [*Nachbesprechung Aufgaben*][t4], [Module und `use`][s10] | [YouTube](https://youtu.be/04dpIX5njy8) | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?id=96da9ffc-91eb-4d90-b89b-11696580e084) |
| 23.11. | [Module, Crates und Cargo][s10], *Farbiger Calculator* | [YouTube](https://youtu.be/mpfAaSVTe78) | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?id=bac86875-bbeb-42dc-9970-55af51c9f017) |
| 28.11. | [Generics und Traits][s11] | [YouTube](https://www.youtube.com/watch?v=ImIKRRXd3fA) | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?id=7859a587-aa3c-470e-8f17-34d62dc66984) |
| 30.11. | [Generics und Traits][s11], [*Generic `read` Beispiel*][m2] | [YouTube](https://www.youtube.com/watch?v=QUWfNqC-7nI) | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?id=2c961a8f-e86e-4335-9a6f-90e8a6d435f5) |
| 05.12. | [*Nachbesprechung Aufgaben*][t6], [Iteratoren und Closures][s12] | [YouTube](https://youtu.be/YnYKzpmMv40) | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?id=e4db0cac-c233-4ae7-9c62-5cd1d6e2ab16) |
| 07.12. | [Iteratoren und Closures][s12], [*Kommaliste Beispiel*][m3] | [YouTube](https://www.youtube.com/watch?v=Ku0001U4o9A) | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?id=5c64074c-80ff-40ce-84ee-5dc4cb349ff5) |
| 12.12. | [`try!` und `?` (Error Handling)][s9], [Closures][s12], [*Themenideen Abschlussprojekt*][m4] | [YouTube](https://www.youtube.com/watch?v=-tc0tHWuJm8) | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?id=7da62492-0c14-41e5-b9a5-ad00dfece221) |
| 14.12. | *Kekse und Coding ("Advent of Code")* | - | - |
| | | | |
| 02.01. | **Fällt aus** | - | - |
| 04.01. | [*Blödsinn*][b1], [*Nachbesprechung Aufgaben*][t8], [Makros][s13] | [YouTube](https://youtu.be/nPF-0zE5i9Q) | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?id=8d44d64b-ac8b-42e0-80dc-223f83ba9745) |
| 09.01. | [*Nachbesprechung Aufgaben*][t9], [Deref und Diverses][s14], [Low Level][s15] | [YouTube](https://youtu.be/TdJKs_Dq2Cs) | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?id=8f7ff3e0-389d-4c98-82ac-5d708b4a1aaa) |
| 11.01. | [Stack und Heap (Low Level)][s15] | [YouTube](https://youtu.be/Unda8t9cxw8) | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?id=31d04ddc-80a3-4344-a2c4-b4a0316f2e3a) |
| 16.01. | [*Nachbesprechung Aufgaben*][t10], [Trait Objects, Drop, Smartpointer][s16] | [YouTube](https://youtu.be/vjQ7qx6RcRY) | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?cid=f4a7c0754b566ef1f43345adaecc98cf&id=8e21ce68-1029-42fd-974d-52dd05a7bb5e) |
| 18.01. | *Informationen Abschlussprojekt*, [Interior Mutability, GC vs. RAII][s16] | [YouTube](https://www.youtube.com/watch?v=1rTtz7qHW68) | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?cid=f4a7c0754b566ef1f43345adaecc98cf&id=1b01a22e-42f7-4528-901f-df41557b4a05) |
| 23.01. | [*Animal Beispiel*][m5], [*Tipps für Abschlussprojekt*][m6], [Lifetimes][s17] | [YouTube](https://youtu.be/1ND79YMDV54) | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?cid=f4a7c0754b566ef1f43345adaecc98cf&id=cd0ead3a-6417-4119-9a82-0419024150e9) |
| 25.01. | [Lifetimes][s17], [Multithreading & Concurrency][s18] | [YouTube](https://youtu.be/RTCHFlGg5wQ) | - |
| 30.01. | *Organisatorisches*, [*Ferris*][m7], [Multithreading & Concurrency][s18] | [YouTube](https://youtu.be/hwoEtAnTuss) | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?cid=f4a7c0754b566ef1f43345adaecc98cf&id=46fff460-84e6-4134-a77b-73e6bfad5f61) |
| 01.02. | [Performance & Effizienz][s19] | [YouTube](https://www.youtube.com/watch?v=01HVacgLQO4) | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?id=84b2b573-e900-4692-bdab-7cea4fd8c332) |
| 06.02. | [Rust Community & Open Source][s20] | [YouTube](https://youtu.be/mKE7j5TC-2Q) | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?id=ba079eff-2e2b-49cb-9cc0-15138e1da23e) |
| 08.02. | [Unsafe & FFI][s21] | [YouTube](https://youtu.be/5IJr7wn48_I) | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?id=6603a519-d899-4c2b-ac25-a4143b7377fa) |



[s0]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/slides/00-Warum-Rust.pdf
[s1]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/slides/01-Grundlagen.pdf
[s2]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/slides/02-Git-GitHub-Rust-Environment.pdf
[s3]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/slides/03-Ownership-System.pdf
[s4]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/slides/04-Strings.pdf
[s5]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/slides/05-Git-Teil-2.pdf
[s6]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/slides/06-Structs-Methoden.pdf
[s7]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/slides/07-Vec-Konstanten-TypeAlias-TupleStruct-Match-Pattern.pdf
[s8]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/slides/08-Enums-Option-Result.pdf
[s9]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/slides/09-Error-Handling.pdf
[s10]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/slides/10-Module-Crates-Cargo.pdf
[s11]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/slides/11-Generic-Traits.pdf
[s12]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/slides/12-Iterators-Closures.pdf
[s13]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/slides/13-Makros.pdf
[s14]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/slides/14-Deref-und-Diverses.pdf
[s15]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/slides/15-Low-Level.pdf
[s16]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/slides/16-TraitObjects-Smartpointer-InteriorMutability-Drop-RAII.pdf
[s17]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/slides/17-Lifetimes.pdf
[s18]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/slides/18-Concurrency-Multithreading.pdf
[s19]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/slides/19-Performance-und-Effizienz.pdf
[s20]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/slides/20-Rust-Community-Open-Source.pdf
[s21]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/slides/21-Unsafe-und-FFI.pdf

[m1]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/materialien/tree.rs
[m2]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/materialien/read.rs
[m3]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/materialien/comma-list-iter.rs
[m4]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/materialien/Zusammenfassung-und-Abschlussprojekt.pdf
[m5]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/materialien/animal.rs
[m6]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/materialien/tipps-abschlussprojekt.md
[m7]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/materialien/ferris.jpg

[t3]: https://github.com/LukasKalbertodt/programmieren-in-rust/tree/master/aufgaben/sheet3
[t4]: https://github.com/LukasKalbertodt/programmieren-in-rust/tree/master/aufgaben/sheet4
[t6]: https://github.com/LukasKalbertodt/programmieren-in-rust/tree/master/aufgaben/sheet6
[t8]: https://github.com/LukasKalbertodt/programmieren-in-rust/tree/master/aufgaben/sheet8
[t9]: https://github.com/LukasKalbertodt/programmieren-in-rust/tree/master/aufgaben/sheet9
[t10]: https://github.com/LukasKalbertodt/programmieren-in-rust/tree/master/aufgaben/sheet10

[b0]: http://i.imgur.com/9g5gebP.jpg
[b1]: https://www.youtube.com/watch?v=d8q5C7UalZw&list=PL5YlUpv9iiO0WsosBlAZfL64BBCALECkW


## License

### Code

All code in this repository (including the code in the slides) is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Slides

All non-code content in this repository (most notable the slides) are licensed under [CC-BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
shall be licensed as above, without any additional terms or conditions.
