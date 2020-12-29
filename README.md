# Iron Oxide
Iron Oxide is a mini web-browser written entirely in rust.

My goal is to make it a command line based web browser, similar to Lynx. But in
the future I would like to improve Iron Oxide and give it graphical support.  
As of now, it can make a single http/s GET request, and can parse the response
body as html and print the result as markdown formatted text

## The current state of Iron Oxide
Iron Oxide only has support for a single GET request, and assumes that the
response is a 200 (OK) response.  
The body of this response is parsed using [`Pest`] which is a rust based parser
that uses parsing expression grammars as input. The result of this parsing is a
Document Object Model (DOM), which is a tree structure used by all web browsers
to display, manipulate, and update a webpage.  
Iron Oxide has no support for javascript or CSS, or any kind of media (images,
videos, figures, svg).  
Iron Oxide has two display modes currently, a text based mode, and an Ncurses
based mode (which currently doesn't do anything). Both of which are controlled
with command line arguments.

## Roadmap
- ~~Provide a text-only based display mode, that formats output in markdown~~
- ~~Provide an Ncurses based display mode (Using [`Rustbox`])~~
    - NOTE: Element compatibility can be improved, but currently it's mostly
usable and I am happy with that
- Improve the current http library to include other request methods (POST, PUT,
OPTIONS, etc.)
- Improve the current http library to include support for all http status codes
(I won't even try to list them all)
- Create a "browser" mode that holds state, manages tabs, etc. etc.  
...
- Provide a Graphical display mode
- CSS Eventually
- Javascript engine from scratch

[`Rustbox`]: https://github.com/gchp/rustbox
[`PEST`]: https://pest.rs
