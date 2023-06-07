# Codebook-2

**This README is under construction!**

Codebook 2 is the second iteration of my programmable stack-based (RPN) CLI calculator. It essentially has its own interpreted scripting language called Codebook Scripting Language (CSL).
that's used to write functions and interact with the stack. **Disclaimer: I am not formally trained on how to program in Rust, nor do I have
a background in compilers, file-lexing, etc.; this is a personal project I made in my free time. I do not have time to dedicate to bug-fixing or
adding new features. In this case, my concern is not runtime or memory efficiency, rather my main concern is ease of computation, which I hope this calculator provides.**

# Postfix (RPN) Calculators

This calculator uses Reverse-Polish Notation (RPN) instead of regular infix notation. for example, 5 + 5 would be written as 5 5 +.

## This Calculator

This calculator runs as a REPL by default, but you can run scripts by entering ```$ cargo run -- f <filepath>```. These examples use the REPL feature.

When doing a calculation, the last number on the stack is printed. For example:
```CSL
[In] << 5 5 +
[Out] >> 10
```
the input is ```5 5 +```, and the result, as printed in the ```[Out] >>``` line, is 10.
```CSL
[In] << 5 6
[Out] >> 6
```
In this case, since no operation was done on the 5 and 6, the last item on the stack is outputted, which happens to be 6.
Codebook 2 also supports string literals:
```CSL
[In] << "Hello, world!" puts
Hello, world!
```
above is a simple "Hello, world!" program.

It's important to note that you can use ```puts``` on things without quotations, but it will not support escapes:
```CSL
[In] << Hello\nworld puts
Hello\nworld
[In] << "Hello\nworld" puts
Hello
world
```
so using quotation marks is encouraged.

# Codebook Scripting Language

## Overview

The scripting language, which is known as Codebook Scripting Language (CSL), is a postfix/stack-based language, meaning it feels like you're 
writing the language backwards, in a sense, or at least in this case.

For example, here is an if-elif-else block:

```CSL
-sall
a 0 def
b 1 def
c 2 def
{ 
  "Not implemented.\n" puts
} a b == if {
  "Not implemented.\n" puts
} b c == elif {
  "Not implemented.\n" puts
} else
```

Notice how the if, elif, and else come after the code that will run if the condition is true, and the ```puts``` comes after the string it 
will print.

## Convention

It's important to know the convention of CSL for organizational purposes. Some things to note are:
* Functions: user-defined function names end with a '!'. There is nothing in place to enforce this, but I think it's good practice.
* Variables: user-defined variable names are all uppercase.
Keywords have nothing special about them; they're all lowercase, and that's why it's important to use these conventions in order
to differentiate Codebook keywords from user-defined function and variables. 

## Features

### Flags

In the application itself, you can enter "help" which will list all keywords divided into categories, with some exceptions\*. Among those, you'll see 
**FLAGS**. As of now, the only flags are -s and -sall, which mean "suppress" and "suppress all" respectively. When a statement like ```a 0 def```
is executed, a message is printed:

```CSL
[In] << a 0 def
[Out] >> a = 0
```

The -s flag removes the ```[Out] >> ``` output once, and -sall will prevent all outputs in the code block. For example:

```CSL
[In] << -s a 0 def b 0 def
[Out] >> b = 0
```

-s only prevented a = 0 from being printed, as opposed to:

```CSL
[In] << -sall a 0 def b 1 def

[In] << 
```

where all outputs were suppressed.

\* exceptions to this are the keywords: namespace, puts, pushch, and " because they are new additions.

### Loops

Loops are imperfect in CSL--there is no functioning while loop or for loop, only a ```times``` loop. Essentially, this means you can have certain code
run a specific amount of times. For example:

``CSL
{
  { + } size! 1 - times
} sum! fn
```

is the function defintion for the ```sum!``` function. It sums every item on the stack. ```{ + } size! 1 - times``` uses the ```+``` operator the size of the stack minus one times, where ```size!``` is essentially a macro for the ```STACK_SIZE``` variable which holds the size of the stack.

### Conditionals

As was previously seen, Codebook 2 supports if-elif-else statements. Essentially, when an if/elif is encountered, it checks the last number on the stack.
* If it's 1: the associated code block is exectuted, and
* If it's 0: execution continues.
Codebook provides equality operators that return either 0 or 1: ==, !=, <, >, <=, >=, where 1 = true and 0 = false.
There is no boolean primitive in Codebook 2.

```
{
  "5 equals 2!\n" puts
} 5 2 == if {
  "5 does not equal 2.\n" puts
} 5 2 != elif {
  "Unreachable\n"
} else
```

Output:

```
5 does not equal 2.
```

### Keywords

Codebook 2 has a large collection of keywords, in fact, it probably has too many. I will go over some of the more important ones.

#### dup

_Duplicates the last number on the stack_
```
[In] << 5 dup +
[Out] >> 10
```

#### drop

_Deletes the number at the top of the stack_
```
[In] << 5 0 5 drop
[Out] >> 0
```

#### def

_Defines a variable_
```
[In] << A 200 def
[Out] >> A = 200
```

## Codebook 3 is under development!

Codebook Version 3 will have an improved lexer for better error-reporting and nicer-looking syntax; its goal is to feel more like a programming 
language than before.
