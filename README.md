# rust-dijkstra (Тороидальная разметка пути)

Небольшая CLI-программа: читает 2D-карту из stdin, ищет любой путь от `i` (старт) к `O` (цель) с тороидальной топологией (переход через края), помечает путь точками `.` (не затирая `i` и `O`) и выводит карту. Если пути нет — печатает исходную карту без изменений.

## Входные данные

- Прямоугольная текстовая сетка
- `#` — стены (непроходимо)
- Пробел ` ` — свободная клетка
- `i` — старт, `O` — цель
- Движение по 4 направлениям, переход через края (тор)

Пример ввода:

```
##    #
#  #i #
#  O## 
   #   
```

Пример вывода:

```
##... #
#  #i #
# .O## 
  .#   
```

## Сборка и запуск

```bash
cargo build
```

По умолчанию чтение из stdin. Примеры:

- POSIX-оболочки:

```bash
cargo run --release - < src/resources/input.txt
```

- PowerShell:

```powershell
Get-Content -Raw src/resources/input.txt | cargo run --release -
```

Также можно передать путь к файлу аргументом:

```bash
cargo run --release src/resources/input.txt
```

## Тесты

```bash
cargo test
```

## Примечания

- Используется BFS; кратчайший путь не обязателен, но обычно таким и будет по шагам.
- Вывод сохраняет форму и пробелы исходной сетки.

## Conventional Commits (рекомендуется)

- feat: read map from stdin and find toroidal path
- test: add unit tests for path finding and wrapping
- docs: add README with usage and examples
