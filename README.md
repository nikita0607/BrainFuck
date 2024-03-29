# BrainFuck

```Да да, это интерпритатор brainfuck'а```

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

Пример вызова функции по значению в ячейке
```bash
++++()>++++()
```

Функции имеют доступ только к своим локальным ячейкам памяти.
```bash
(.)+++++++++++++++++++++++++++++++++()  # Ничего не выведится, потому что в локальной ячейки функции ничего нет
```

Код для функций также может описываться в скобках, которые её вызывают:
```bash
(.)(+++++++++++++++++++++++++++++++++)  # Выведеться '!', так как мы взаимодействуем с локальной ячейкой
```
### Фигурные скобочки
Фигурные скобки описывают область кода, похожую на ту, что описываются круглыми скобками, но ячейки памяти в этой области общие для всех таких областей кода:
```bash
{+++++++++++++++++++++++++++++++++}{.}  # '!'

({+++++++++++++++++++++++++++++++++})(){.}  # '!'
```
Функции в фигурных скобках можно создать только в рамках одного блока кода:
```bash
{(++++++++++++++++++++++++)}{(++++++++++++++++++++++)}
#  Во втором блоке фигурных скобок создается новая функция
```

Также можно использовать вложенные фигурные скобки:
```bash
{{}}
```

Вложенные скобки будут работать с ячейками памяти той области кода, в которой находятся фигурные скобки:
```bash
+++++++++++++++++++++++++++++++++{{.}}  # '!'

(+++++++++++++++++++++++++++++++++{{.}})()  # '!'
```

Из вложенных фигурных скобок нельзя вызвать функии, объявленные в области кода, к которой они принадлежат:
```shell
(+++++++++++++++++++++++++++++++++.){{()}}  # Ничего не выведится
```

### Комментарии

Вы можете писать любые символы, кроме символов языка BrainFuck без изменения
поведения программы. Но, если вы хотите написать символ BrainFuck'а, что бы он
не влиял на работу программы, вы можете поставить перед этим символом знак '`':
```shell
+++++++++++++++++++++++++++++++++`.  # Ничего не выведется
```

### Доступные флаги
```'-h': Выводит информацию по доступным флагам```\
```'-f PATH': Запускает файл```
