<div align="center">
  <img src="images/omenspace.png" alt="Logo de OMEN Space" width="120" />

  # OMEN Space

  **El centro de control para Linux, ligero y completo, para HP Omen, Victus y Transcend.**  
  *Escrito íntegramente en Rust para un rendimiento nativo con GTK4 y sin sobrecarga.*

  [![Version](https://img.shields.io/badge/Release-v2.1.8-red.svg?style=flat-square)](https://github.com/yunusemreyl/omen-space/releases)
  [![License](https://img.shields.io/badge/License-GPL%203.0-green.svg?style=flat-square)](LICENSE)
  [![Platform](https://img.shields.io/badge/Platform-Linux-lightgrey.svg?style=flat-square)]()
  [![Built with Rust](https://img.shields.io/badge/Language-Rust-orange.svg?style=flat-square)]()

  [English](README.md) · **Español**
</div>

> [!IMPORTANT]
> **Estado — avanzando a v2.1.8.** La versión v2.1.8 publicada **se está actualizando** y se refrescará cuando terminen las pruebas actuales. Hasta entonces, las correcciones y funciones indicadas aquí solo existen en la rama `main`.
> **Recomendamos a todos instalar la versión de vanguardia (canary)** para obtener las correcciones de v2.1.8 — consulta la [instalación rápida](#-instalación-rápida). Informa de cualquier problema en el [seguimiento de incidencias](https://github.com/yunusemreyl/omen-space/issues).

---

## ✨ Características

OMEN Space ofrece todo lo necesario para aprovechar al máximo tu portátil en Linux, sin añadidos innecesarios.

- 🎛️ **Control de ventiladores y temperatura:** crea curvas de ventilador personalizadas para un funcionamiento casi silencioso sin limitación térmica. Incluye un **modo de limpieza de ventiladores** y **detección automática de parada de ventiladores**.
- 🎮 **Superposición HUD rápida (Shift+F2):** HUD flotante GTK4 para cambiar al instante los modos de ventilador y energía con telemetría en vivo sin abrir la aplicación principal.
- ⚡ **Perfiles de rendimiento:** cambia entre los modos ACPI/WMI `power-saver`, `balanced` y `performance` con protección activa keep-alive de EC y respaldo automático `WMBA` → `WMAA`.
- 🚀 **Intel RAPL, Ryzen SMU y undervolt:** límites PL1/PL2 de Intel RAPL (hasta 150W/200W en procesadores HX y Core Ultra), undervolt directo por MSR, control del desfase TCC, límites de TGP de la GPU y ajuste del SMU de AMD Ryzen.
- 🎮 **Interruptor MUX:** cambio nativo de Optimus / enrutamiento de la dGPU para el máximo rendimiento en juegos.
- 🔍 **Interfaz escalable (Ctrl+Mayús+Más / Ctrl+Mayús+Menos):** amplía toda la ventana entre el 80 % y el 200 % (`Ctrl+Mayús+0` restablece) para que la aplicación siga siendo legible en monitores grandes o de alta densidad sin pasar a pantalla completa. El factor se recuerda y también puede ajustarse en Ajustes → Apariencia.
- 🔋 **Telemetría de batería:** la carga y la salud de la batería se muestran en el monitor del sistema, el HUD rápido y la bandeja.
- 🔔 **Alertas térmicas más discretas:** las notificaciones son transitorias (no se acumulan en el historial), se reemplazan entre sí y solo aparecen si la temperatura alta dura unos segundos. La protección del ventilador sigue reaccionando al instante.
- 🌈 **RGB Studio:** iluminación de teclado de 4 zonas, zona única, barra de luz o por tecla USB HID (`0d62:54bf`), con efectos de onda, respiración, ciclo y estático. En teclados Darfon por tecla, un interruptor **experimental de *Efectos por hardware*** permite que el propio MCU del teclado ejecute arcoíris, ciclo, respiración, starlight y raindrop con un uso de CPU casi nulo (el MCU ignora el control de brillo en estos efectos; ver [#322](https://github.com/yunusemreyl/omen-space/issues/322)).
- 🔄 **Comprobador inteligente de BIOS:** consulta los servidores de HP para buscar el firmware más reciente de tu placa.

---

## ⚡ Instalación rápida

La forma más sencilla de instalar OMEN Space es con el instalador web de una línea. Detecta tu distribución, gestiona las dependencias y lo compila todo automáticamente.

**Recomendado por ahora — versión de vanguardia (canary), incluye todas las correcciones de v2.1.8:**
```bash
curl -sSL https://raw.githubusercontent.com/yunusemreyl/omen-space/main/install.sh | sudo bash -s -- --canary
```

**Versión estable** (se actualizará tras las pruebas; *aún no* incluye las últimas correcciones de v2.1.8):
```bash
curl -sSL https://raw.githubusercontent.com/yunusemreyl/omen-space/main/install.sh | sudo bash -s -- --stable
```

> **Nota:** si ejecutas el instalador sin parámetros, te pedirá elegir el canal de forma interactiva.

📖 **Guía completa en español** (requisitos, dependencias, solución de problemas): [docs/INSTALACION.es.md](docs/INSTALACION.es.md)

<details>
<summary><b>📦 Otros métodos de instalación (manual, AUR, NixOS)</b></summary>

**Con Git (Ubuntu/Debian, Fedora, Arch, openSUSE):**
```bash
git clone https://github.com/yunusemreyl/omen-space.git
cd omen-space
sudo ./setup.sh install
```

**Arch Linux (PKGBUILD):**
```bash
git clone https://github.com/yunusemreyl/omen-space.git
cd omen-space
makepkg -si
```

**NixOS (Flakes):**
```bash
nix profile install github:yunusemreyl/omen-space
```

**Desinstalar:**
Si instalaste con el script de instalación rápida o con `setup.sh`:
```bash
cd omen-space
sudo ./setup.sh uninstall
```
*(Si borraste la carpeta, ejecuta de nuevo `git clone https://github.com/yunusemreyl/omen-space.git` antes).*
</details>

---

## 🆕 Novedades de v2.1.8 (en `main`)

- **Corregido:** la interfaz se cerraba sola cada ~30 s (fallo del heartbeat) y la desincronización del panel de acceso rápido.
- **Corregido:** falsas alertas de sobrecalentamiento por picos breves de CPU; las notificaciones son transitorias, con retardo y reemplazables.
- **Corregido:** la potencia total ya no cuenta los marcadores de GPU inactiva; se informa la carga de la GPU; una temperatura de GPU no disponible muestra `--`; la etiqueta FAN muestra correctamente *Custom* y *EC*.
- **Nuevo:** telemetría de batería, atajos de escalado de la interfaz, efectos por hardware experimentales para teclados Darfon por tecla.

## 🧪 Se busca ayuda

- **Propietarios de OMEN MAX 16 (8D41 / 8D87):** prueba *Efectos por hardware* en RGB Studio y cuéntanos en [#322](https://github.com/yunusemreyl/omen-space/issues/322).
- **Verificación de placas:** abre una incidencia *"Verify my laptop"* con el informe de diagnóstico y tu `board_name`.

---

## 📸 Capturas de pantalla

<p align="center">
  <img src="images/perf.png" width="48%" alt="Rendimiento y editor de curva de ventiladores" />
  <img src="images/appprofiles.png" width="48%" alt="Perfiles por aplicación" />
</p>
<p align="center">
  <img src="images/keyboard.png" width="48%" alt="Ajustes RGB" />
  <img src="images/undervolt.png" width="48%" alt="Undervolt en Ryzen" />
</p>

<details>
<summary><b>🔍 Ver más capturas (MUX, diagnóstico, ajustes, actualizador, CLI)</b></summary>
<br>
<p align="center">
  <img src="images/mux.png" width="48%" alt="Interruptor MUX" />
  <img src="images/diagnostic.png" width="48%" alt="Diagnóstico" />
</p>
<p align="center">
  <img src="images/settings.png" width="48%" alt="Ajustes" />
  <img src="images/updater.png" width="48%" alt="Actualizador" />
</p>
<p align="center">
  <img src="images/cli.png" width="48%" alt="CLI" />
</p>
</details>

---

## 🏗️ Arquitectura

OMEN Space es una reescritura completa del antiguo *OmenCtl* en Python, ahora en **Rust**, con una huella de ~3 MB, menos de 5 MB de RAM y respuesta instantánea.

- **`omen-space-daemon`**: el backend. Se ejecuta como servicio de systemd (root) y gestiona las interacciones con WMI, ACPI, Sysfs y MSR a través de D-Bus.
- **`omen-gui`**: un frontend GTK4 + Libadwaita muy rápido, que se ejecuta en espacio de usuario.
- **`omen-overlay`**: la superposición HUD rápida (`Shift+F2`) bajo demanda en GTK4.
- **`omen-tray`**: un applet ligero para el panel del escritorio que permite cambiar rápido de perfil.
- **`omen-cli`**: una interfaz de terminal rápida y programable, con un comando para el HUD (`omen-cli overlay toggle`).
- **`hp-omen-extra`**: el controlador de kernel DKMS que amplía las capacidades estándar del kernel.

---

## 🌐 Idioma de la aplicación

La interfaz está disponible en **inglés, turco y español**. Cambia el idioma en *Ajustes → Apariencia e idioma*. Con la opción automática se usa el idioma del sistema.

¿Quieres añadir otro idioma? Las cadenas están en `src/omen-gui/src/i18n.rs` (una función `translate_*` por idioma). Las normas de contribución están en [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md).

---

## 👨‍💻 Créditos y licencia

OMEN Space se distribuye bajo la licencia **GPL-3.0** y lo impulsa una comunidad increíble.

- **[yunusemreyl](https://github.com/yunusemreyl)** - Desarrollador principal
- **[tuxov](https://github.com/tuxov)** - Responsable del módulo de kernel

Gracias a todos los colaboradores: [@aloshy0](https://github.com/aloshy0), [@CodesRahul96](https://github.com/CodesRahul96), [@xcellsior](https://github.com/xcellsior), [@TitoTFP](https://github.com/TitoTFP), [@SafSaf0999](https://github.com/SafSaf0999), [@yijean34-source](https://github.com/yijean34-source), [@ManOfDiamond](https://github.com/ManOfDiamond), [@ASPRNG-PRGMR](https://github.com/ASPRNG-PRGMR), [@WoofahRayetCode](https://github.com/WoofahRayetCode), [@henfiber](https://github.com/henfiber) (revisiones detalladas), [@ktarxhun](https://github.com/ktarxhun), y a los proyectos `omencore`, `omen-rgb-keyboard` y [`omen-rgb-linux`](https://github.com/arfelious/omen-rgb-linux) (protocolo del MCU Darfon).

*Aviso: OMEN Space es un proyecto independiente y NO está afiliado ni respaldado por HP.*
