# SubSens

**SubSens** is a simple and lightweight mod that allows even lower sensitivity in Minecraft.

It is especially useful when you want to have the exact **same sensitivity across different games**, but you play at **high DPI with low in-game sensitivity**.

This is the **Horizon mod API** port, for launcher **0.2.70** and later. The
launcher replaced the old `.hnmod` / horizon-mixins system with the new native
mod API; the old port is kept in [`old-hnmod/`](old-hnmod/) and no longer loads.
The original Java/Fabric mod is kept in [`old-java/`](old-java/).

## Features

- A sensitivity setting from `-1/3` to `1`
- `-1/3` is true zero camera movement
- An on/off toggle
- No separate config files; the setting is on **Options → Mods → SubSens**
- No dependencies beyond the Horizon mod SDK

## How it works

The game turns mouse movement with `(sensitivity * 0.6 + 0.2)^3`. At the vanilla
minimum of `0.0` that curve is still `0.008`; SubSens can go down to `-1/3`,
where the curve reaches zero.

The mod uses the mod API's `calculate_player_turn` hook. That hook is told the
vanilla mouse sensitivity for the turn, and lets a mod replace it. SubSens
replaces it with its own setting, so the whole range is available without
patching the vanilla options screen.

The setting is a slider on the mod's page. A step of `0.0001` keeps the value
precise enough for normal DPI conversion; the exact value is stored by the
game's mod settings system, not in `options.txt`.

## Calculating the correct value

Use a **sensitivity converter that allows negative Minecraft sensitivity values**,
or use the formula:

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

Set that value on **Options → Mods → SubSens**. The nearest slider step is
`-0.0154`.

## Requirements

The mod API is in Horizon **main since 2026-11-16**. It needs a game build with
the mod API and its SDK — the first Nightly after that merge, or a developer
build with an SDK. The old Nightly **2026-11-13 (`9f42064`)** predates the mod
API and cannot load this mod.

## Build and install

The mod source is in [`horizon-mod/`](horizon-mod/).

```sh
cd horizon-mod
cargo horizon-mod check
horizon-launcher native-build . --instance "<instance>" --trust-native-code
```

`cargo horizon-mod check` fetches the pinned SDK into `.horizon/sdk/` and
checks the package. The launcher builds the mod against the prebuilt game and
makes that build the instance's game. Native code runs with your rights, so
read the source before you allow it.

To make a distributable package:

```sh
cargo horizon-mod build
# target/horizon-mod/subsense.hmod
```

## Development

The project is a flat native mod under `horizon-mod/`:

```text
horizon-mod/
  horizon-mod.toml     the mod's manifest and build pins
  Cargo.toml           depends on the SDK's horizon-mod-api
  src/lib.rs           Hooks: register + calculate_player_turn
  assets/subsense/lang/en_us.json
  icon.png
```

The manifest pins Horizon `baea2549…` and Horizon Mixins `e59da909…`. The port
uses only the mod API (no `[native.*]` tables), so a newer build can run it when
the API items it uses are unchanged. After fetching the SDK, the pure tests run
with:

```sh
cargo test
```

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
