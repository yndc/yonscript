# Yonscript

Yonscript is a simple strongly-typed markup and scripting language designed for defining game resource data and logic. 

Yonscript code is interpreted by the Yonscript virtual machine written in *TODO: language*.

## Goal

The goal for this project is to create a scripting language for defining game resource data and logic, therefore these are some important requirements for the language:

### Simple and easy to learn

Not everyone that adds content into the game has a programming background. The language must be designed so that anyone could edit the values without much confusion when editing the code. 

### Serializable

The source code must be able to be serialized into structured data that is easily manipulated by a tool such as visual code editor.

### Portable

The source code must be portable to any major gaming platforms such as desktop PCs, consoles, and mobiles. And the code must be fully decoupled with a game engine implementation. Thereforce any source code must be able to be interpreted on runtime.

This is also designed to decouple game content with platform-specific distribution solution such as Steam and App Store, as content updates might be handled in-app and no app update from the distribution solution is necessary for updating the game content.

### Highly performant

As the code could be running on runtime inside a game that could be running on a beefy desktop or an old smartphone, performance is a top priority. 

To achieve this, we decided some design decisions to make the engine as efficient as possible while maintaining portability and ease of use:

- Strongly-typed by default
- Wrote the VM in C++
- Supports simple promise-based multi-threading primitives
