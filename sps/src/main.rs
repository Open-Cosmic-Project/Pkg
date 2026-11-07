//! SPS — Secure-Package-System
//!
//! Alcance acordado en CosmicOS spec secciones 6, 7.5 y 8:
//! - Gestor de paquetes oficial para aplicaciones `.sps`.
//! - Comando: `sps --create-package <nombre-del-paquete> <directorio-del-paquete>`.
//! - Genera y valida `Manifest.json` (encabezado, cuerpo, cola con hash firmado).
//! - Empaqueta binarios compilados y recursos en imágenes EROFS firmadas para /usr/apps/<id>.

use std::env;

fn main() {
    println!("[SPS] Secure-Package-System — Gestor de paquetes de CosmicOS");

    let args: Vec<String> = env::args().collect();
    if args.len() >= 4 && args[1] == "--create-package" {
        let pkg_name = &args[2];
        let pkg_dir = &args[3];
        println!("[SPS] Creando paquete '{}' a partir del directorio '{}'...", pkg_name, pkg_dir);
    } else {
        println!("Uso: sps --create-package <nombre-paquete> <directorio-paquete>");
        println!("     sps install <archivo.sps>");
    }
}
