# Overview

I really wanted to learn about rust and so I made a few simple projects here. I had read that rust was the most loved programming language for the sixth year in a row this year and wanted to learn more about it. The capstone of what I did here is creating an program that interacts with an sql database. This is contained in the sqlite folder. 

The program allows the user to make and filter through requests. The program only shows the user requests that pertain to them.

Overall I wanted to learn how to use sql and get used to that and decided to do it in a new language I hadn't used before. 

[Software Demo Video](https://youtu.be/ROkJea4_EvY)

# Relational Database

The database is designed to manage requests that the user has created and allow people with proper credentials to view them. 

The database contains two main tables. One for credentials so that users can make accounts and only be shown information that is relevant to them. The other contains requests that different users have filled out. 

# Development Environment

I did this project in the rust language. I used the rusqlite crate to handle interactions with the database. This crate is designed specifically for sqlite databases. I also used the rand crate to generate random numbers and the chrono crate to log the data that entries were made. 

# Useful Websites

* [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/intro.html)
* [Rust Cheat Sheet](https://cheats.rs/)
* [Rust Documentation](https://doc.rust-lang.org/reference/introduction.html)
* [Rust Getting Started](https://doc.rust-lang.org/book/)

# Future Work

* Add a status item that can be updated by supervisors to requests
* Create a non-command-line interface
* Add data sorting for when there are more entries.