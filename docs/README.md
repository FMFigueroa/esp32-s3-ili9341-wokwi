# ESP32-S3 + Display TFT ili9341 + Wokwi Simulator

<p align="center">
<img  src="https://res.cloudinary.com/rustlatamgroup/image/upload/v1677900599/Rust%20Embedded/02%20display/02_eb4y0z.png">
</p>
<hr/>

### [Demo](https://www.youtube.com/watch?v=Ig-lr3LtoDc)
Bienvenido a todos, Rust se ha convertido en una solución para diversos sectores del desarrollo de software y la tecnología, en especial para los sistemas embebidos, esta es una prueba del simulador Wokwi usando el ESP32-S3 con un display ILI9341 conectado por SPI, es una aplicacion bare metal no-std, para más detalle visita [Rust on ESP Community](https://github.com/esp-rs). Espero que te sirva como guía de aprendizaje y pueda ayudarte al máximo para que te abra las puertas a nuevas oportunidades como desarrollador de Rust Embedded🦀.

<hr>

## Dev Containers

This repository offers Dev Containers supports for:

- [Gitpod](https://gitpod.io/)
- ["Open in Gitpod" button](https://www.gitpod.io/docs/getting-started#open-in-gitpod-button)
- [VS Code Dev Containers](https://code.visualstudio.com/docs/remote/containers#_quick-start-open-an-existing-folder-in-a-container)
- [GitHub Codespaces](https://docs.github.com/en/codespaces/developing-in-codespaces/creating-a-codespace)
  > **Note**
  >
  > In order to use Gitpod the project needs to be published in a GitLab, GitHub,
  > or Bitbucket repository.
  >
  > In [order to use GitHub Codespaces](https://github.com/features/codespaces#faq)
  > the project needs to be published in a GitHub repository and the user needs
  > to be part of the Codespaces beta or have the project under an organization.

If using VS Code or GitHub Codespaces, you can pull the image instead of building it
from the Dockerfile by selecting the `image` property instead of `build` in
`.devcontainer/devcontainer.json`. Further customization of the Dev Container can
be achived, see [.devcontainer.json reference](https://code.visualstudio.com/docs/remote/devcontainerjson-reference).

When using Dev Containers, some tooling to facilitate building, flashing and
simulating in Wokwi is also added.

### Build

- Terminal approach:

  ```
  scripts/build.sh  [debug | release]
  ```

  > If no argument is passed, `release` will be used as default

- UI approach:

  The default build task is already set to build the project, and it can be used
  in VS Code and Gitpod:

  - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Build Task` command.
  - `Terminal`-> `Run Build Task` in the menu.
  - With `Ctrl-Shift-B` or `Cmd-Shift-B`.
  - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Task` command and
    select `Build`.
  - From UI: Press `Build` on the left side of the Status Bar.

### Flash

> **Note**
>
> When using GitHub Codespaces, we need to make the ports
> public, [see instructions](https://docs.github.com/en/codespaces/developing-in-codespaces/forwarding-ports-in-your-codespace#sharing-a-port).

- Terminal approach:

  - Using `flash.sh` script:

    ```
    scripts/flash.sh [debug | release]
    ```

    > If no argument is passed, `release` will be used as default

- UI approach:
  - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Task` command and
    select `Build & Flash`.
  - From UI: Press `Build & Flash` on the left side of the Status Bar.
- Any alternative flashing method from host machine.

### Wokwi Simulation

#### VS Code Dev Containers and GitHub Codespaces

The Dev Container includes the Wokwi Vs Code installed, hence you can simulate your porjects by:

1. Press `F1`
2. Run `Wokwi: Start Simulator`

For more information and details on how use the Wokwi extension, see [Getting Started] and [Debugging your code] Chapter of the Wokwi documentation.

[getting started]: https://docs.wokwi.com/vscode/getting-started
[debugging your code]: https://docs.wokwi.com/vscode/debugging

> **Warning**
>
> ESP32-C2 is not supported in Wokwi yet.

> **Warning**
>
> Gitpod does not, yet, support Wokwi extension hence Wokwi simulation is not available in Gitpod

#### Gitpod

`wokwi-server` is part of the Gitpod image so you can run:

```sh
wokwi-server --chip <chip> <pathToElf>
```

If you want to run your binary in a custom Wokwi project:

```sh
wokwi-server --chip <chip> --id <projectId> <pathToElf>
```

> **Warning**
>
> The simulation will pause if the browser tab is in the background.This may
> affect the execution, specially when debuging.

##### Debuging with Wokwi in Gitpod

Wokwi offers debugging with GDB.

- Terminal approach:

  ```
  $HOME/.espressif/tools/xtensa-esp32s3-elf/esp-2021r2-patch3-8.4.0/xtensa-esp32s3-elf/bin/xtensa-esp32s3-elf-gdb target/xtensa-esp32s3-espidf/debug/esp32_s3_with_ili9341 -ex "target remote localhost:9333"
  ```

  > [Wokwi Blog: List of common GDB commands for debugging.](https://blog.wokwi.com/gdb-avr-arduino-cheatsheet/?utm_source=urish&utm_medium=blog)

- UI approach:
  1. Run the Wokwi Simulation in `debug` profile
  2. Go to `Run and Debug` section of the IDE (`Ctrl-Shift-D or Cmd-Shift-D`)
  3. Start Debugging by pressing the Play Button or pressing `F5`
  4. Choose the proper user:
     - `esp` when using VS Code or GitHub Codespaces
     - `gitpod` when using Gitpod

<hr>
