Introduction
===

At this point you have learned the essentials of Substrate runtime development and how to bootstrap a UI for your runtime.

This next section will cover writing tests for your Substrate runtime. Tests are the backbone of any software artifact, ensuring its veracity as your codebase evolves.

While you can test some of your code logic through the UI, it is best practice to write unit tests for each runtime module. Unit tests should cover the core functions of your runtime.

Conveniently, Substrate has some low-level primitives that help you execute your runtime tests with minimum hassle, meanwhile giving it capabilities such as calling other modules and processing blocks to be as realistic as possible.