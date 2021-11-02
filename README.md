# BrainFuck

Да да, это интерпритатор brainfuck'а

### Как это использовать

Запустите исполняемый файл интепритатора, и пишите!

### Пример программы
``` bash
+++++++++++++++++++
```

### Еще пример
```bash
++++++++++++++++++++++++++[.-]
```

### И на последок
```bash
+++++++++++++++++++(++++++++++++++[.-])+++++++++()
```

### Круглые скобки
`Ого, круглые скобки! А что это?`

Круглые скобки - это подобие функций из более "самостоятельных" ЯП.
Для дальнейшего вызова этой "функции" будет использоваться текущее значение из
ячейки памяти. Что бы создать функцию, нужно написать круглые скобки:
```bash
()
```
А для вызова этой функции нужно написать круглые скобки:
```bash
()
```

В итоге получиться такой код:
```bash
()()
```

Функции имеют доступ только к своим локальным ячейкам памяти.
```bash
(.)+++++++++++++++++++++++++++++++++()  # Ничего не выведится, потому что в локальной ячейки функции ничего нет
```

Код для функций также может описываться в скобках, которые её вызывают:
```bash
(.)(+++++++++++++++++++++++++++++++++)  # Выведеться '!', так как мы взаимодействуем с локальной ячейком
```

