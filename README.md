# Abont

## Preface

This is a design document for Abont: a new take on programmer-oriented interface, a superset of
shells, terminals, orthodox file managers and code editors. If you are thinking "but this is Emacs,
no?", you are not wrong.

This is something I'd be building if I had a spare couple of years. As I don't have spare capacity
for this endeavor, but can't help thinking about this, I'll restrain myself to writing this design
document. I don't know how complete it'll end up being!

If you are excited about these ideas, please feel free to start hacking! Maybe there are enough
like-minding people around that we could bootstrap a community without any single individual
spending years on it?

## Introduction

Abont is a new API to develop "line of business" applications against, where business is programming.
It's a different "thin waist". Today we have two popular "thin waists" for programming tools:

* GUI: a **canvas of pixels** in a window.
* Terminal: a **textual stream** with escape sequencies to modify behavior of the stream.

Similarly, Abont is a **set of annotated text buffers**, arranged in a tiled display. That is:

The foundation is text, like with the terminal. Unlike the terminal, it is not an append-only text
stream, but rather a mutable text buffer. An application which displays the text buffer can update
the buffer in the middle.

The text is "rich", in a sense that the spans of text are annotated with attributes. For example, a
span of text can be annotated with a hyper-link. Buttons, checkboxes and other interactive elements
are also implemented as clickable text.

Unlike full HTML, the text is not nested, and logically is a 2 dimensional grid of characters. This
makes _text_ navigation (kbd>Home</kbd>, <kbd>End</kbd>, <kbd>PgUp</kbd>, and
<kbd>PgDown</kbd>) universally applicable.

Another different from a terminal is that there are _multiple_ text buffers. You can have several
apps running at the same time and displaying a text buffer each, or you could have a single app that
owns multiple buffers.

Buffers are arranged in splits and tabs. Applications have _limited_ control over the spatial
arrangement of the buffers. It is the user who chooses which buffers are displayed in the
foreground.

This is how Abont should look like:

![Magit status screenshot](https://matklad.github.io/assets/magit.png)

This is a screenshot of excellent Magit interface from Emacs. The maximalist goal of Abont is to
ensure that all interactive programming tools _start_ with a magit-like interface, that this kind of
experience becomes the default, and that this kind of experience is easily emendable into different
tools, the same way most code editors embed terminals.

## Design

This sections collects various micro essays on various aspects of the architecture.

### Extensibility Vs Composability

Emacs is extensible --- you can change the way Emacs behaves by writing some lisp. Unix shell is
composable: you can extend it by combining existing processes in a new way or writing new programs.
It is worth noting that composability is nothing more than extensibility on the next level. A shell
is composable because UNIX is exentisble -- you can extend your UNIX environment with new program,
which you can then compose in a shell.

It seems that the sweet spot is to straddle to levels --- have _both_ composability across processes
and extensibility within the process. Vim, Emacs, and shell all have a scripting engine, and
affordances for outsorting work to external processes.

Where Emacs falls short, I think, is in not exposing Emacs date model to ,external world. As far as
I know, there's no easy way to implement Magit as a separate process.

So, one specific technical goal of Abont is to introduce an IPC protocol to expose a **set of
attributed text buffers** across the process boundary.

At the same time, for small-scale extensibility and private scripting, in-process scripting language
would be more convenient.

### Scripting Language

As far as I am aware, there's no suitable scripting language for Abont. The following two design
criteria exclude all popular languages, as far as I am aware:

* Static (but maybe gradual) type system. The tooling unlocked by static typing is just too
  valuable. It is much easier to extend something if you can auto-complete your way through the API.
* Good support for utf-8 text. Obviously, if our model is text buffers, we should be good at text!

The _closest_ language would be TypeScript. I'd take all JavaScript quirks (even IEEE754) if I also
get TypeScript-level tooling. But utf-16 strings feel like a hard blocker. Can we hack some JS
interpreter to use utf8?

Alternatively, perhaps "just compile to WASM" is an answer? Doesn't seem so, as there are at least
two blockers:

* WASM still lacks component model (I think?). Building compenents on the level of byte buffers is
  too low-level.
* There isn't a default choice of a WASM-compiled scripting language. Even if we use WASM, we'd
  still want 90% of code to be in the same language. Rust feels a bit too low-level to write your
  `init.abont` in.

Decision: keep it Rust. We _really_ need a proper high-iteration-speed scripting language here, but
there isn't one, and building our own is a yak too hairy. Instead the plan:

* Make strict separation about the API (window creation, etc), the engine realizing API in a form of
  graphical window, and the `abont` implementation, which uses the API, but doesn't depend (even
  transitively!) on the engine.
* At some point later, allow user-scripts, by compiling them to .so/.wasm on the fly.
* In the far future, pick a real scripting language to slowly transition to.

### Open-vs-Closed API

For in-process extensibility, the two related questions are:

* open API (Emacs, IntelliJ) vs closed API (VS Code)
* image based programming (Emacs, Pharo, Erlang) vs traditional "run this source code" programming
  (everything that's not smalltalk or a lisp machine)

Open API means that there isn't a strict distinction between the platform, and the extension.
Extensions can touch and change everything, which is very powerful, but can easily break things or
makes things slow.

Closed API means that the platform exposes a bridge specifically for extensions. This restricts the
power of extensions but also makes evolving API much easier. Additionally closed API could be much
easier to learn. `vscode.d.ts` is brilliant --- the entire API surface as a single self-contained
file!

Given that IPC API is effecively closed, it might makes sense to go for open API for the `abont`
binary? If we code ourself into backwards compatability corner with the internal API, we can declare
bankruptcy and start `abont2`, getting to re-use all out of process components! It would perhaps be
best to even _start_ with a couple of different `abont` implementations?

Image based programming is when the bug you are chasing is in the code that no longer exists as a
source code. I am pretty convinced that serializing everything as text is the way to go, and that
there shouldn't be much support for image based programming outside of tightly scoped live-reload (
it is a research question whether tightly scoped live-reload and image based programming are in fact
distinct things).

### Extensions

We'd rather want to be like VS Code marketplace, rather than like Emacs wiki. Or rather, we want to
be like Go: everything is decentralized and can be hosted whatever, but there's also default caching
service which guarantees some amount of availability and also provides some measure of
discoverability. Just leverage crates.io?

### GUI

Would be great to pick a GUI lib that can do rich text out of the box! No idea what's the right
choice here in 2024.

### Compat

It would be cool to lift usual terminal applications into `abont` world. That doesn't feel _too_
hard --- there needs to be a wrapper that creates a pty pair and adapts the output to `abont` IPC.
Sadly, this seem to really require going all the way down to the kernel for a pty-pair! Horrible.
Abont itself clearly should be very cleanly virtualizable. It would be disgusting to have abont
server implemented as a terminal program!

### Remoting

We are obviously living in the future, so I should be able to run a local abont on my laptop and use
that command the laptop, the server runnning in the basement and an ephemeral machine on the other
side of the cloud.

Crucially, abont IPC protocol itself _shouldn't_ be a thin waist for remoting. I should be running
the shell locally, and, when I run `ls` alternative, it should execute the logic on my machine. But
it obviously should run `readdir` syscall on a remote machine, and shuttle only the results back.

Is this in scope of Abont even? Maybe not!

### Applications

Things which I think are going to be hard without something like `abont`:

* Non-blocking shell: when you spawn `cargo build`, it is detached by default. Immediately after,
  you can spawn another `cargo build`, or do a `git commit`, while the original `cargo build` is
  still running. Its output is clearly visible, but doesn't get in a way. Similarly, each command
  gets stamped with its start and end time.

* Concurrent shell: if I need to spawn a cluster of tree programs, I shouldn't jump through hoops to
  separate the three outputs into different streems. I shouldn't do anything at all for that
  purpose, in fact: waiting for process to finish should be a special case, a-la Rust inert async
  case.

* Command palette front and center: when I am typing a shell command, I don't want to look at the
  lower-left corner of my screen. The thing should be front and center.

* Magit as a separate program

Things which are not innovative per-se, but which would be required to actually use the thing:

* Basic shell to run programs. Maybe a shell language?
* Basic text editor (replacing something like Zed or VS Code is a non-goal, at least until it is
  proven that abont model works)
* File browser.

## Reference

### Data Model

That's actually the main thing to fill out! Feel free to send PRs!

```rust
// State

/// Singleton repressing the entire abont process. This probably corresponds to a single window.
/// Do we want to have abont spawning multiple windows? Probably, but I think it would be OK to cut
/// that, at least initially.
struct Abont {
    prompt: Prompt,
    splits: Vec<Split>,
    split_arrangement: SplitArrangement
    buffers: Vec<Buffer>,
    documents: Vec<Document>,
}

/// Prompt is a special singleton split used for the primary interraction with the user.
/// Think command palette, `M-x`, or, indeed, shell's prompt. Maybe we want to display it at the
/// bottom, like in Emacs, or maybe we want to popup it front and center.
struct Prompt {
    buffer: BufferRef
}

/// A single part of split-screen display that displays a single buffer.
struct Split {
    buffer: BufferRef
}

/// I don't know how to arrange splits on the screen! The natural implementation is to just do a
/// binary tree, but it feels like it would suck?
enum SplitArrangementBinary {
    Leaf(SplitRef)
    VSplit(SplitArrangementBinary, SplitArrangementBinary)
    HSplit(SplitArrangementBinary, SplitArrangementBinary)
}

/// Maybe this is better?
enum SplitArrangement {
    Leaf(SplitRef),
    Split {
      direction: Vertical | Horizontal,
      splits: Vec<SplitArrangement>,
    }
}


/// A Buffer is its textual content plus extra state, notably, cursors.
/// Do cursors belong in the core model? I think so, they are the primary means of interaction.
/// Though, it's a bit hard to see how to make Vim vs Emacs bindings customizable without
/// hard-coding?
struct Buffer {
    document: DocumentRef,
    selection: Selection
}

struct Selection {
  ranges: PointRange,
}

struct PointRage {
  start: Point,
  end: Point,
}

/// A single document could be shown in several buffers
struct Document {
  text: AText
}

/// Logically, this is attributed text! Have no idea how to represent it physically.
struct AText {
    text: String,
    attrs: Vec<Attribute>
}

struct Attribute {
    range: PointRange,
    value: AttributeValue,
}

// Operations
impl Abont {
  fn create_buffer() -> BufferRef;
}

impl Document {
  fn replace(&mut self, selection: SelectionRequest, replacement: AText) {}
}

enum SelectionRequest {
  Everything,
  Start,
  End,
  Selection(Sellection), 
}

```

### IPC

TBD

## Prior Art

* Acme: https://research.swtch.com/acme.pdf.

  It seems _very_ close to what I want here, with three major differences:

    * I am not a big fan of mice, I'd love to keep things keyboard driven
    * Acme is plain-text editor (not even syntax highlighting!). It works by heuristically
      identifying text as file names, etc. I think it makes more sense to follow Emacs model and
      endow the text with attributes!
    * It only has "external" extensibility. To me, it seems that you'd want to both script the
      system from inside, as well as extract larger things into separate processes. The API should
      be the same either way! External/Internal is the question of distribution, not interaction!

* Emacs: magit, dired, eshell
* terminal.click
* Arcan's cat9
* warp (though they don't make any attempts to actually move the state of the art, and rather just
  pile more hacks on the sandy foundation)
* kitty on tmux

## Naming

```console
$ sha256sum abont
9fce2fc695ad8dcda4c6e3dcb1842be801c0bd9808c0af36ae963f62d3494349  abont
```
