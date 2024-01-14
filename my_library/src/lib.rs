/// # My Library
///
/// `my_library` est une bibliothèque qui fournit une fonction pour effectuer l'addition de deux nombres.

/// Additionne deux nombres.
///
/// Cette fonction prend deux nombres de type `usize` et retourne leur somme.
///
/// # Exemples
///
/// ```
/// use my_library::add;
/// let result = add(2, 2);
/// assert_eq!(result, 4);
/// ```

/// `add` - Additionne deux nombres.
///
/// # Paramètres
///
/// * `left`: Le premier nombre à additionner. Il doit être de type `usize`.
/// * `right`: Le deuxième nombre à additionner. Il doit être de type `usize`.
///
/// # Retour
///
/// Retourne la somme de `left` et `right` sous forme de `usize`.
///
/// # Exemples
///
/// ```
/// use my_library::add;
/// let result = add(2, 2);
/// assert_eq!(result, 4);
/// ```

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Teste la fonction `add`.
    ///
    /// Ce test vérifie que la fonction `add` retourne correctement la somme de deux nombres.
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
