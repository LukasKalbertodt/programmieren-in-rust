Aufgabe 1: Sleep Sort
=====================

Sortieralgorithmen sind ja immer toll. Da wir jetzt Multithreading können, wollen wir "Sleep Sort" implementieren. Der Trick ist:

- Für jeden Integer im zu sortierenden Array wird ein Thread gestartet
- Jeder Thread schläft entsprechend seinem Integer lange
- Sobald der Thread aufwacht, pusht er seinen Integer auf ein Ergebnis-Array

Danach sollte das Ergebnis-Array theoretisch sortiert sein. Um zu untersuchen, wie weit Theorie und Praxis voneinander abweichen, prüfen wir ab welchen Zeitdimensionen der Scheduler genau genug wird:

Zuerst soll jeder Thread sein Element in *Nano*sekunden schlafen. Nanosekunden sind eher kurz und hier wird man bei einem Array wie `[83, 12, 13, 35, 91]` noch keinen Erfolg haben. Wenn wir nach dem ersten Versuch feststellen, dass das Ergebnis-Array nicht sortiert ist, versuchen wir es noch mal. Diesmal soll aber jeder Thread zwei mal sein Element in Nanosekunden schlafen.

Entsprechend erhöhen wir die Zeit, für die geschlafen werden soll, mit jedem Versuch um Faktor 2. Euer Programm soll am Ende ausgeben, wie viele Versuche gebraucht wurden und welcher Zeit-Multiplikator schließlich ausreichte, um das Array zu sortieren.
