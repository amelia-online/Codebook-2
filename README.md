# Codebook-2

Codebook 2 is the second iteration of my programmable stack-based (RPN) CLI calculator. It essentially has its own (unnamed) interpreted scripting language 
that's used to write functions and interact with the stack. **Disclaimer: I am not formally trained on how to program in Rust, nor do I have
a background in compilers, file-lexing, etc.; this is a personal project I made in my free time. I do not have time to dedicate to bug-fixing or
adding new features. In this case, my concern is not runtime or memory efficiency, rather my main concern is ease of computation, which I hope this calculator provides.**

# The Scripting Language

## Overview

The scripting language, which for now I'll just call TSL (The Scripting Language), is a postfix/stack-based language, meaning it feels like you're 
writing the language backwards, in a sense, or at least in this case.

For example, here is an if-elif-else block:

```TSL
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

It's important to know the convention of TSL for organizational purposes. Some things to note are:
* Functions: user-defined function names end with a '!'. There is nothing in place to enforce this, but I think it's good practice.
* Variables: user-defined variable names are all uppercase.
Keywords have nothing special about them; they're all lowercase, and that's why it's important to use these conventions in order
to differentiate Codebook keywords from user-defined function and variables. 

## Features

### Flags

In the application itself, you can enter "help" which will list all keywords divided into categories, with some exceptions\*. Among those, you'll see 
**FLAGS**. As of now, the only flags are -s and -sall, which mean "suppress" and "suppress all" respectively. When a statement like ```a 0 def```
is executed, a message is printed:

```TSL
[In] << a 0 def
[Out] >> a = 0
```

The -s flag removes the ```[Out] >> ``` output once, and -sall will prevent all outputs in the code block. For example:

```TSL
[In] << -s a 0 def b 0 def
[Out] >> b = 0
```

-s only prevented a = 0 from being printed, as opposed to:

```TSL
[In] << -sall a 0 def b 1 def

[In] << 
```

where all outputs were suppressed.

\* exceptions to this are the keywords: namespace, puts, pushch, and " because they are new additions.

### Loops

Loops are imperfect in TSL--there is no functioning while loop or for loop, only a ```times``` loop. Essentially, this means you can have certain code
run a specific amount of times. For example:

```TSL
{
  { + } size! 1 - times
} sum! fn
```

is the function defintion for the ```sum!``` function. It sums every item on the stack. ```{ + } size! 1 - times``` uses the ```+``` operator the size of the stack minus one times, where ```size!``` is essentially a macro for the ```STACK_SIZE``` variable which holds the size of the stack.
