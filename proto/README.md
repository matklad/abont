Prototype Abont implementation. Again, I don't intend to spend a lot of time on this, but I just
can't help sketching at least some code...

* `abont-api` this crate defines the thin waist I've been talking in the top-level README. This is
  _just_ API, without its user or implementer.

  Note that attributed text as a value type is a part of these API layer (the same way a `String`
  would be).

  `atext` module is worth noting --- it implements attributed text, and needs _a lot_ of work.

* `abont-shell` is the user of the API. While the plan is to have many users communicating over IPC,
  let's start with something simple, just a basic shell/editor/file browser. Shouldn't be hard,
  right?

* `abont-egui` this is the other side of API, the thing that shows pixels. Obviously, the real
  implementation would use a real GUI here, with hand written shaders, vulkan and what not. I am not
  particularly interested in GUIs though, so this is intended to be just a placeholder implemented
  with egui (I picked egui because it has rich text as an example on the main page).

  When we finish prototyping, we'll replace this thing whole-sale. That's why it is crucially
  important that there aren't any dependencies between `abont-shell` and `abont-egui`

* `abont` --- neither shell nor gui depend on each other, but there needs to be _something_ to bind
  them to together. `abont` is this thing, it has a trivial main that just plugs `shell` into
  `egui`.
