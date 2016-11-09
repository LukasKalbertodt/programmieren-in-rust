Aufgabe 3.2: Pokemon
====================

In dieser Aufgabe soll ein kleines, Terminal-basiertes Pokemon-Spiel
programmiert werden.
Dieses Spiel werden wir in den nächsten Übungszetteln stetig erweitern.

Um direkt mal etwas zu zeigen: *In etwa* so wird das Ergebnis dieser Aufgabe
aussehen:

```
Player Red, please choose a Pokemon (or type '?' to get a complete list)
?
#001 Bulbasaur
#002 Ivysaur
#003 Venusaur
#004 Charmander
#005 Charmeleon
#006 Charizard
#007 Squirtle
#008 Wartortle
#009 Blastoise
Player Red, please choose a Pokemon (or type '?' to get a complete list)
Charmander
Player Blue, please choose a Pokemon (or type '?' to get a complete list)
Wartortle
>>>>> Status: Charmander has 18 HP, Wartortle has 20 HP
>>>>> Charmander is about to attack! Which move shall it execute?
    0: Tackle
    !!! Please give me the attack ID:
0
>>>>> Charmander uses Tackle! (Wartortle has 15 HP left)
Wartortle is about to attack! Which move shall it execute?
    0: Tackle
    1: Water Gun
    !!! Please give me the attack ID:
1
>>>>> Wartortle uses Water Gun! (Charmander has 7 HP left)
>>>>> Status: Charmander has 7 HP, Wartortle has 15 HP
Charmander is about to attack! Which move shall it execute?
    0: Tackle
    !!! Please give me the attack ID:
0
>>>>> Charmander uses Tackle! (Wartortle has 10 HP left)
Wartortle is about to attack! Which move shall it execute?
    0: Tackle
    1: Water Gun
    !!! Please give me the attack ID:
0
>>>>> Wartortle uses Tackle! (Charmander has 1 HP left)
>>>>> Status: Charmander has 1 HP, Wartortle has 10 HP
Charmander is about to attack! Which move shall it execute?
    0: Tackle
    !!! Please give me the attack ID:
0
>>>>> Charmander uses Tackle! (Wartortle has 5 HP left)
Wartortle is about to attack! Which move shall it execute?
    0: Tackle
    1: Water Gun
    !!! Please give me the attack ID:
1
>>>>> Wartortle uses Water Gun! (Charmander has 0 HP left)
>>>>> Charmander fainted!
```


## Umfang und Art der Aufgabe

In dieser Aufgabe wird bereits recht viel Code bereitgestellt und insgesamt
wird die Lösung eher lang. Derzeit ist der komplette Code noch in einer Datei;
das wird sich aber in der nächsten Woche ändern, wenn wir Module kennenlernen.

Trotzdem soll diese Aufgabe schulen, sich in Rust-Code zurechtzufinden. Falls
einige Sachen unklar sein sollten, zögert nicht, auf Piazza zu fragen! Viele
gute Fragen auf Piazza helfen auch den anderen.

Außerdem geht es in dieser Aufgabe natürlich um Pokemon. Ich habe versucht,
alles so zu formulieren, dass es auch Menschen verstehen, die nie etwas mit
Pokemon am Hut hatten. Falls mir das irgendwo nicht gelungen ist, sagt mir
bitte Bescheid oder fragt direkt auf Piazza. "Pokemon" stand nämlich nicht als
Voraussetzung in der Kursbeschreibung :P

Aber grob beschrieben: In Pokemon gibt es komische Tiere/Monster, die "Pokemon"
genannt werden. Diese Pokemon können von Menschen gefangen werden, meist um
damit gegen andere "Pokemon-Trainer" zu kämpfen. Es gibt unterschiedliche
Pokemon und unterschiedliche Attacken.

In der bereitgestellten Datei gibt es bereits diversen Code. Davon besteht ein
größerer Teil nur aus Konstanten. Das sind einfach Daten über Pokemon und
Attacken, die nicht weiter verstanden werden müssen. Wichtig ist aber, dass
all diese Daten direkt in der Executable gespeichert sind und daher immer die
`'static` Lifetime haben.


## a) Pokemon wählen lassen

In diesem Teil geht es darum, die beiden Spieler zu fragen, welches Pokemon sie
benutzen möchten. Dazu sollt ihr ein paar Funktionen erstellen:

- `print_pokemon_list`: Druckt eine Liste aller verfügbaren Pokemon (in der
  `POKEDEX` Konstante gespeichert) auf dem Terminal.
- `find_pokemon_by_name`: Bekommt einen Namen und sucht nach einem PokemonModel
  mit diesem Namen im `POKEDEX`. Gibt das PokemonModel zurück, oder `None`,
  wenn der Name nicht gefunden wurde. Die Suche darf gerne in O(n) sein.
- `choose_pokemon`: Fordert den Spieler auf, den Namen eines Pokemon einzugeben.
  Bietet außerdem an, alle Pokemon aufzulisten, wenn der Spieler '?' eingibt.
  Diese Funktion liefert dann ein PokemonModel zurück (falls der Spieler
  einen ungültigen Namen eingibt, erneut auffordern, einen richtigen Namen
  einzugeben). Hierfür ist die Funktion `read_string()` nützlich!

Wenn diese Funktionen funktionieren, soll `choose_pokemon()` in der `main()`-
Funktion aufgerufen werden, sodass beide Spieler ein Pokemon wählen können.



## b) `Pokemon` Typ implementieren

Bis jetzt gibt es nur einen Typ `PokemonModel`, der globale Eigenschaften von
einer Pokemon-Art speichert (wie z.B.: "Pferde haben vier Beine").
Allerdings brauchen wir noch einen Typ, der eine Instanz eines Pokemon
darstellt (wie z.B. "dieses Tier ist ein Pferd und hat braunes Fell").
Dazu soll ein neuer Typ `Pokemon` angelegt werden.

Dieser Typ speichert sich folgende Daten:

- Zu welcher Pokemon-Art er gehört (Referenz auf ein `PokemonModel`; wichtig:
  alle PokemonModels haben eine `'static` Lifetime).
- Die derzeitigen "stats" (Typ `Stats`), also z.B. auch die HP
- Das derzeitige Level (mögliche Level: 1 bis 100)

Der Typ soll folgende Funktionen und Methoden besitzen:

- Konstruktor-Funktion `with_level`: Gegeben wird ein PokemonModel und ein
  Level, eine gültige `Pokemon`-Instanz soll zurückgegeben werden. Hinweis:
  Nützlich ist die Funktoin `Stats::at_level()`.
- Getter-Methoden (in Rust verzichtet man auf `get_`, also nicht `get_foo`,
  sondern oft nur `foo`):
  - `stats()`
  - `model()`
  - `name() `
  - `level()`
- `is_alive()`
- `endure_attack()`: Bekommt eine Referenz auf ein anderes Pokemon und auf
  eine Attacke. Abhängig davon werden die HP von dem jetzigen Pokemon
  angepasst. Ihr braucht die Funktion `attack_damage()` dafür.


## c) Kampfsystem

Zuletzt müssen wir nur noch das eigentliche Kampfsystem bauen. Dazu sollen
abwechselnd beide Spieler aufgefordert werden, eine Attacke auszusuchen.
Diese Attacke wird dann vom Pokemon ausgeführt und so das andere Pokemon
verletzt. Die ungefähren Anforderungen:

- Beide Spieler werden abwechselnd gefragt
- Es werden die verfügbaren Attacken aufgelistet
- Die gewählte Attacke wird ausgeführt
- Der Status beider Pokemon wird regelmäßig angezeigt
- Wenn ein Pokemon stirbt, soll sich das Programm beenden

Euer Programm muss nicht genau so aussehen, wie oben im Beispiel gezeigt.

Bonusaufgabe: Zuerst darf das Pokemon mit dem höheren `speed` Wert angreifen.
