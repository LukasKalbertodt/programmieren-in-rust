Aufgabe 1.1: Git und Rust einrichten
====================================

In dieser Aufgabe geht es lediglich darum, das Git-Repository richtig
einzurichten.


## Zugriff auf Repository bekommen

Zunächst muss ich euch ein Repository erstellen. Dazu müsst ihr euch zuerst im
Testattool eingetragen haben und mir dann eine E-Mail schreiben. Die E-Mail
sollte lediglich folgenden Inhalt haben:

```
pir-task-{{a_rz}}-{{b_rz}}
{{a_gh}}
{{b_gh}}
```

Hierbei stehen `{{a_rz}}` und `{{b_rz}}` für eure RZ-Logins. Ein Beispiel:
`pir-task-wwacker-ssorglos`. `{{a_gh}}` und `{{b_gh}}` sind eure
GitHub-Accountnamen (z.B. WilliWacker und SusiSorglos). Falls ihr noch keinen
GitHub-Account habt, müsst ihr euch einen erstellen.

Nachdem ihr mir diese E-Mail geschickt habt, sollte ich irgendwann dazu kommen,
euch ein Repository zu erstellen und euch Zugriff darauf zu gewähren. Darüber
werdet ihr von GitHub per E-Mail benachrichtigt.


## Forken und Klonen

Sobald ihr die Einladung akzeptiert und Zugriff auf das Repository habt, müsst
ihr dieses Repository *forken*. Das geschieht durch den Klick auf den
Fork-Button direkt auf GitHub. Diesen Fork müsst ihr nun auf euren lokalen PC
klonen, sodass ihr lokal arbeiten könnt. Dies passiert in etwa so:

```
git clone git@github.com:WilliWacker/pir-task-wwacker-ssorglos.git
```

Die URL findet ihr direkt auf GitHub. Stellt sicher, dass ihr die SSH-URL
benutzt (und nicht HTTPS). Neue GitHub-Benutzer müssen sich sehr wahrscheinlich
einmal durch [dieses Tutorial][ssh-key-help] durcharbeiten, um einen SSH-Key
einzurichten.

Vergewissert euch außerdem, dass in der URL, die ihr klont, kein
"LukasKalbertodt" zu finden ist. Ihr müsst euren Fork klonen, nicht das
originale Repository, was unter meinem Account zu finden ist.

Wenn das alles geklappt hat, solltet ihr euer fertig geklontes Repository
sehen; darin findet ihr einen Ordner `ci`, sowie ein paar andere Dateien.

[ssh-key-help]: https://help.github.com/articles/adding-a-new-ssh-key-to-your-github-account/


## Die eigentliche Aufgabe

Verändert die beiden Dummy-Namen in der `README.md` zu euren richtigen Namen.
Diese Veränderung muss in einem Commit festgehalten werden, welchen ihr dann
schon auf GitHub pushen sollt. Danach könnt ihr bereits auf GitHub einen
PR öffnen, um zu sehen, ob alles klappt.

Wenn alles gut aussieht, könnt ihr die zweite Aufgabe bearbeiten. Die Commits
der zweiten Aufgabe pusht ihr einfach auch auf GitHub – der PR wird
automatisch aktualisiert.
