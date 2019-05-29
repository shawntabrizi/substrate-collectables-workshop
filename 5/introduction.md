Introduction
===

At this point you have learned the essentials of Substrate runtime development and how to bootstrap a UI for your runtime.

This next section will cover writing tests for your Substrate runtime. Tests are the backbone of any software artifact, ensuring its veracity as your codebase evolves. There are many _point-of-views_ when it comes to writing tests. We are not really focused on that on that in the scope of this tutorial. Rather, we will focus on the technical details of writing tests in Substrate.

While you can test some of your code logic through the UI, it is best practice to write unit tests for each runtime module. Unit tests should cover the core functions of your runtime.

Conveniently, Substrate has some configurations that help you execute your runtime tests with minimum hassle.