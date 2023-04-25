# Codebook-2

Codebook 2 is the second iteration of my programmable stack-based (RPN) CLI calculator. It essentially has its own (unnamed) interpreted scripting language 
that's used to write functions and interact with the stack. **Disclaimer: I am not formally trained on how to program in Rust, nor do I have
a background in compilers, file-lexing, etc.; this is a personal project I made in my free time. I do not have time to dedicate to bug-fixing or
adding new features.**

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
## Features

In the application itself, you can enter "help" which will list all keywords divided into categories. Among those, you'll see 
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
