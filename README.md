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
