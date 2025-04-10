# Pokémon Breeding System
A simple command-line application for managing a Pokémon breeding center, written in Rust.

## Features
- Add new Pokémon with name, type, and gender
- Display all Pokémon in the breeding center
- Train all Pokémon by gaining experience points (XP)
- Breed compatible Pokémon to create new Pokémon
- Sort Pokémon by level or type
- Save Pokémon data to a file for persistent storage

## Requirements
- Rust (latest stable version recommended)
- `rand` crate for random number generation

## Installation
1. Clone this repository or download the source code.
2. Install Rust if you haven't already (https://www.rust-lang.org/tools/install).
3. Build the project:
   ```
   cargo build --release
   ```
4. Run the application:
   ```
   cargo run
   ```

## Usage
The application provides a simple menu-driven interface:
1. **Show All Pokémon** - View a list of all Pokémon in the breeding center.
2. **Add a New Pokémon** - Add a Pokémon with details (name, type, gender).
3. **Train All Pokémon** - Increase the experience points (XP) of all Pokémon.
4. **Try Breeding** - Breed two compatible Pokémon to create a new Pokémon.
5. **Sort by Level** - Sort Pokémon by their level in descending order.
6. **Sort by Type** - Sort Pokémon alphabetically by their type.
7. **Save to File** - Save the current Pokémon data to a file.
8. **Exit** - Close the application.

## Data Storage
Pokémon data is saved to a file named `pokemons.txt` in the same directory as the application. This file is created or overwritten when you choose the "Save to File" option.

## Technical Details
The application uses:
- `rand` crate for generating random genders and names for baby Pokémon during breeding.
- Standard Rust file I/O operations for saving Pokémon data.
- A simple command-line interface for user interaction.

## Project Structure
- `main.rs` - Contains the main application logic, including the menu and user interaction.
- `pokemons.rs` - Defines the `Pokemon` struct, `BreedingCenter` struct, and their associated methods.

## Future Improvements
- Add more Pokémon types and attributes.
- Implement advanced breeding mechanics (e.g., inheritance of traits).
- Add a graphical user interface (GUI) for better user experience.
- Enable loading Pokémon data from a file on startup.