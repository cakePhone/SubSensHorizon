# SubSens

**SubSens** is a simple and lightweight mod that allows even lower sensitivity in Minecraft.

It is especially useful when you want to have the exact **same sensitivity across different games**, but you play at **high DPI with low in-game sensitivity**.

This repository now contains the **Horizon** port: a native Rust mod for
[Minecraft: Java Edition 26.3 rewritten in Rust](https://playhorizon.dev). The
original Java/Fabric mod is kept in [`old-java/`](old-java/) for reference.

## Features

- Allows the sensitivity slider to go down to roughly `-67%`
- Stores the precise number directly in `options.txt`
- Supports negative `mouseSensitivity` values
- No separate menus or config files
- No dependencies

## How it works

By default, your mouse sensitivity is somewhere between `0.0` and `1.0`. However, the game's zero is not the **actual zero**.

**SubSens** extends the lower end of the default sensitivity range, allowing you to go all the way down to **true 0 sensitivity**, which means basically **no camera movement at all**.

The game turns mouse movement with `(sensitivity * 0.6 + 0.2)^3`. At the vanilla
minimum of `0.0` that curve is still `0.008`; SubSens moves the minimum to
`-1/3`, where the curve reaches zero. The slider keeps its `0..1` travel and
maps it onto `-1/3..1`, so the percentage beside it can be negative.

## Calculating the correct value (`options.txt`)

Use a **sensitivity converter that allows negative Minecraft sensitivity values**.

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

That is also the reason this mod was created in the first place.

## Install (Horizon)

Build the source-only `.hnmod`:

```sh
python3 tools/package.py
```

It writes `target/subsense-1.0.0.hnmod`. Put that file in Horizon's
`horizon-native-mods/` folder, then enable **SubSens** in Options → Mods and
let the launcher build the native game. Native mods are compiled into a copy of
the game and run with your user rights, so only install source you trust.

The mod is pinned to the Horizon Nightly of **2026-11-08**
(`fdc87aecdbd116bf1e08adcd062fcbaea8c2e2ca`). It changes the vanilla
sensitivity option and the `options.txt` codec, so a different Horizon commit
may need its target lines updated.

## Development

The range math is a standalone Rust library with unit tests:

```sh
cargo test
```

The mixin bundle lives in `mixins/subsense/`. To check it against a pristine
Horizon checkout:

```sh
horizon-mixins plan \
  --workspace /path/to/horizon \
  --mods mixins \
  --out target/plan.json

horizon-mixins check \
  --workspace /path/to/horizon \
  --mods mixins \
  --out target/stage \
  --package horizon-client-render \
  --trust-native-mods
```

The port was tested against the pristine source export of
`fdc87aecdbd116bf1e08adcd062fcbaea8c2e2ca` with horizon-mixins 0.6.0. The
targeted tests cover loading and saving negative values, the exact
`options.txt` round trip, the extended slider range and the untouched-value
preservation.

## Credits

SubSens was created by Maciej "ChikaraDude" Panocha. The original Java/Fabric
mod is at [chikaradude/SubSens](https://github.com/chikaradude/SubSens). This
Horizon port keeps the original MIT licence and is not affiliated with Mojang
or Microsoft.

## AI note

This Horizon port was made with the use of AI, using the model
**DeepSeek V4.1 Flash 1017**.

## Support

**SubSens** is free and open source. If you would like to support the original
project, use [Suppi by Patronite](https://suppi.pl/chikaradude).
