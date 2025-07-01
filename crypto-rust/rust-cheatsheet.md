# Commandes de base Rust / Cargo

## Compilation

- **Build en mode debug**  
  ```sh
  cargo build
  ```
  > Le binaire est généré dans `target/debug/`

- **Build en mode release (optimisé)**  
  ```sh
  cargo build --release
  ```
  > Le binaire est généré dans `target/release/`

## Exécution

- **Lancer l’application (mode debug)**  
  ```sh
  cargo run
  ```

- **Lancer l’application (mode release)**  
  ```sh
  cargo run --release
  ```

## Utilisation du vendor

- **Mettre à jour le dossier vendor**  
  ```sh
  cargo vendor
  ```

- **Compiler en mode offline avec le vendor**  
  (Nécessite le flag nightly ou une config `.cargo/config.toml`)
  ```sh
  cargo build -Z offline
  ```

## Tests unitaires

- **Lancer tous les tests**  
  ```sh
  cargo test
  ```

## Autres commandes utiles

- **Vérifier la compilation sans générer de binaire**  
  ```sh
  cargo check
  ```

- **Formater le code**  
  ```sh
  cargo fmt
  ```

- **Lint (analyse statique)**  
  ```sh
  cargo clippy
  ```
