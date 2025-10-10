# Ana

Ana is meant to be my own lexer/parser library written in Rust.
It is in its early stages, only able to lex and parse a specific subset of TOML, namely the subset relevant to the proxmox answer.toml format.
As it grows and I run into other file formats I want to handle, I will expand on this library. There are some bugs currently. My general approach
to bugs for early projects is to fix the low hanging fruit bugs and then put off more complex ones until I actually run into them while using the library.
This obviously isn't the proper strategy for production software, but at the end of the day, I am the only customer for this software, and I find it of
greater value to myself to ship code than to outline every single sad path possible like I would do for production systems.

## FAQ

**Why did you write your own TOML lexer/parser instead of using serde, toml, etc?**

Writing your own tools is fun and fulfilling, and I haven't made a lexer, parser, interpreter etc in a few years(last time was writing a lisp interpreter in go).
Furthermore, I'm relatively new to Rust and writing a parser is a good project to iterate on while learning as it exposes the developer to a breadth of concepts.

**Will Ana support other file formats later? What about the full TOML spec? I mean the grammar is outlined right on the TOML website!**

Yes, as I run into other file formats that I need to parse or other parts of the TOML spec that I need to process, I will update things to match.
The project is relatively fluid and is meant primarily to fit my needs.

**I stumbled on this repo and want to use it, but I found a bug. What should I do?**

Fork and fix it for yourself, or send me an email at blackswordgroup@gmail.com and I'll try to fix it asap. Althoug honestly if you're already planning
on using an outside dependency for your scanning and parsing needs, you might want to pull in a stable one. But, up to you. If you want the bug fixed for a real use case, I'll fix it.
