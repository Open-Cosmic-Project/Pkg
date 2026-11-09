# Pkg

Gestor de paquetes de CosmicOS.

| Crate | Qué es |
|---|---|
| `sps` | SPS (Secure-Package-System): crea, verifica e instala paquetes `.sps` |

Un paquete `.sps` es un binario nativo más un `Manifest.json` con permisos y bibliotecas mínimas. Para crear uno:

```sh
sps --create-package <nombre> <directorio>
```

## Compilar

```sh
cargo build --release -p sps
```

## Licencia

GPL-3.0-or-later.
