//! # My Library
//!
//! `my_library` est une bibliothèque qui fournit des fonctionnalités pour effectuer des opérations arithmétiques de base.
//!
//! ## Exemples
//!
//! Voici un exemple d'utilisation de la fonction `add` de `my_library` :
//!
//! ```
//! use my_library::add;
//! let a: usize = 1;
//! let b: usize = 2;
//! assert_eq!(add(a, b), 3);
//! ```
//!
//! ## Fonctions
//!
//! - `add`: Cette fonction prend deux nombres et retourne leur somme.

use my_library;

/// # Main
///
/// La fonction `main` est le point d'entrée de notre programme. Elle utilise la fonction `add` de `my_library` pour additionner deux nombres, puis imprime le résultat.
///
/// Ensuite, elle imprime "Hello, world!" à l'écran.

fn main() {
    let a: usize = 1;
    let b: usize = 2;
    println!("{}", my_library::add(a, b));
    println!("Hello, world!");
}
