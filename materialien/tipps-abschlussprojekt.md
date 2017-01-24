Tipps fürs Abschlussprojekt:
============================

### Crates finden

Passende Crates finden ist mit `crates.io` noch ein bisschen schwierig. Daher:

- [**`stdx`**](https://github.com/brson/stdx): Die wohl wichtigsten Rust-Crates. Hier sollte man zuerst schauen, wenn man etwas sucht. Außerdem ist es auch sinnvoll, sich die Crates einfach mal so anzuschauen, um zu sehen, was es schon alles gibt.
- [**Awesome Rust**](https://github.com/kud1ing/awesome-rust): Lange Liste mit Crates zu jedem Thema.


### Travis

Continuous Integration Services sind super hilfreich, gerade bei stark typisierten Sprachen und TDD Projekten. Travis ist ein solcher Service, den ihr kostenlos nutzen und extrem schnell einrichten könnt. Mehr Informationen gibt es z.B. [hier](https://docs.travis-ci.com/user/languages/rust/). Aber in kurz:

1. Eine `.travis.yml` in eurem Repository mit folgendem Inhalt anlegen:

```
language: rust
rust:
  - stable
  - beta
  - nightly
matrix:
  allow_failures:
    - rust: nightly
```

2. Auf `travis-ci.com` mit GitHub Account anmelden und Repository aktivieren.
