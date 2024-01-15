# Dans ce fichier, on peut définir des commandes qui seront exécutées par la commande make.
# Pour exécuter une commande, il suffit de taper make <commande> dans le terminal.
# Par exemple, pour exécuter la commande `cargo build`, il suffit de taper `make build` dans le terminal.
# Pour plus d'informations sur la commande make, voir https://www.gnu.org/software/make/manual/make.html
# Attention : Les séparateurs de ligne sont des tabulations, pas des espaces.

# On définit une commande `help` qui affiche la liste des commandes disponibles.
SHELL := /bin/bash
.PHONY: help

help:
    @grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}

build: ## Build the project
    cargo build

run: build ## Run the project
    ./target/release/your_project_name

test: ## Run the tests
    cargo test

clean: ## Clean the project
    cargo clean

doc: ## Generate the documentation
    cargo doc --no-deps --open

fmt: ## Format the code
    cargo fmt

lint: ## Lint the code
    cargo clippy

check: ## Check the code
    cargo check

update: ## Update the dependencies
    cargo update

release: ## Release the project
    cargo build --release

bench: ## Run the benchmarks
    cargo bench

benchmem: ## Run the benchmarks with memory profiling
    cargo bench --no-run --profile-memory

benchcpu: ## Run the benchmarks with CPU profiling
    cargo bench --no-run --profile-cpu

benchall: ## Run the benchmarks with all profiling
    cargo bench --no-run --profile all