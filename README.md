# Vorlesung „Programmieren in Rust“

Deutsche Vorlesung über Rust an der Universität Osnabrück, Wintersemester 2016/17. In diesem Repository werden Slides und andere Materialien zur Vorlesung gesammelt.

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
| 28.11. | [Generics und Traits][s11] | - | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?id=7859a587-aa3c-470e-8f17-34d62dc66984) |
| 30.11. | [Generics und Traits][s11], [*Generic `read` Beispiel*][m2] | [YouTube](https://www.youtube.com/watch?v=QUWfNqC-7nI) | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?id=2c961a8f-e86e-4335-9a6f-90e8a6d435f5) |
| 05.12. | [*Nachbesprechung Aufgaben*][t6], [Iteratoren und Closures][s12] | [YouTube](https://youtu.be/YnYKzpmMv40) | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?id=e4db0cac-c233-4ae7-9c62-5cd1d6e2ab16) |
| 07.12. | [Iteratoren und Closures][s12], [*Kommaliste Beispiel*][m3] | [YouTube](https://www.youtube.com/watch?v=Ku0001U4o9A) | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?id=5c64074c-80ff-40ce-84ee-5dc4cb349ff5) |
| 12.12. | [`try!` und `?` (Error Handling)][s9], [Closures][s12], [*Themenideen Abschlussprojekt*][m4] | [YouTube](https://www.youtube.com/watch?v=-tc0tHWuJm8) | [via OpenCast](https://video4.virtuos.uos.de/engage/theodul/ui/core.html?id=7da62492-0c14-41e5-b9a5-ad00dfece221) |
| 14.12. | *Kekse und Coding ("Advent of Code")* | - | - |
| | | | |
| 02.01. | **Fällt aus** | - | - |
| 04.01. | tbd | tbd | tbd |


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

[m1]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/materialien/tree.rs
[m2]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/materialien/read.rs
[m3]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/materialien/comma-list-iter.rs
[m4]: https://github.com/LukasKalbertodt/programmieren-in-rust/blob/master/materialien/Zusammenfassung-und-Abschlussprojekt.pdf

[t3]: https://github.com/LukasKalbertodt/programmieren-in-rust/tree/master/aufgaben/sheet3
[t4]: https://github.com/LukasKalbertodt/programmieren-in-rust/tree/master/aufgaben/sheet4
[t6]: https://github.com/LukasKalbertodt/programmieren-in-rust/tree/master/aufgaben/sheet6
