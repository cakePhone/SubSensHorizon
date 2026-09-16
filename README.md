# SubSens

**SubSens** is a simple and lightweight mod that allows even lower sensitivity in Minecraft.

It is especially useful when you want to have the exact **same sensitivity across different games**, but you play at **high DPI with low in-game sensitivity**.

## Features

- Allows sensitivity slider go down to roughly `-67%`
- Stores precise number directly in `options.txt`
- Supports negative `mouseSensitivity` values
- No separate menus or config files
- No dependencies

## How it works

By default, your mouse sensitivity is somewhere between `0.0` and `1.0`. However, the game's zero is not the **actual zero**.

**SubSens** extends the lower end of the default sensitivity range, allowing you to go all the way down to **true 0 sensitivity**, which means basically **no camera movement at all**.

## Calculating correct value (options.txt)
Use some **sensitivity converters that allows negative Minecraft sensitivity values**.

You can also do it manually by using this formula:

```text
newSens = (oldDPI / newDPI)^(1/3) × (oldSens + 1/3) - 1/3
```

Personal use case as an example:

```text
current mouseSensitivity = 0.00066961
current DPI  = 690
desired DPI  = 800
desired mouseSensitivity = -0.01539956185845284
```
That's also the reason why I created this mod in the first place.

## Support
**SubSens** is free and open source.
If you would like to support the project, use [Suppi by Patronite](https://suppi.pl/chikaradude).
