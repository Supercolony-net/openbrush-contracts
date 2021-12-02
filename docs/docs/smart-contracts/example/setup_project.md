---
sidebar_position: 2
title: Setup the project
---

In the first step, we will of course create our project. We will be using several contracts, as defined in the [overview](/smart-contracts/example/overview), so we will put each of them in a separate folder. That means a folder for the fungible token implementation, shares token, loan token, and the lending contract. Each of these folders will contain a `lib.rs` file with the contract's logic, `Cargo.toml` file with the dependencies, and a `.gitignore` file. The structure of the lending contract will be a little more complicated, but we will cover that later.
