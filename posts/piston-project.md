---

title: What is the Piston Project

---

The Piston Project is a collection of game programming libraries that make it
easy to develop games with the Rust programming language.  These libraries are
made to be easily combined with eachother, however it is just as easy to add
non-piston libraries to your stack.

Let's take a look at a few of the libraries in the Piston Project.

## Piston

As any native game developer knows, getting a window open and a triangle on
the screen can be hell.  The titular project __Piston__ helps with the first
of those by providing a clean, backend-independent windowing API.  The API
can be backed by SDL2, GLFW, or any other system that another party
decides to write that implements the __Piston__ API.

Aside from just creating a window, __Piston__ exposes an interface for
keyboard and mouse events, making interacting with the window just as easy
as opening it.

If you decide,

Let's take a look at the code that it takes to get a window up on the
screen.

```rust
extern crate piston;
extern crate sdl2_game_window;

// This would contain our game state
struct App;

// This would contain our game behavior
impl Game for App {}

fn main() {
    let mut window = GameWindowSDL2::new(
        GameWindowSettings {
            title: "test".to_string(),
            size: [800, 400],
            fullscreen: false,
            exit_on_esc: true
        }
    );

    let game_iter_settings = GameIteratorSettings {
        updates_per_second: 120,
        max_frames_per_second: 60
    };
    App.run(&mut window, &game_iter_settings);
}
```

You'll notice that the SDL2 backend was used for this example, how and why
we use it is a bit out of scope for this introduction, but will be covered
by the tutorial.

## Rust-Graphics

__Rust-Graphics__ is a 2d graphics API that can draw to any graphical backend.
Right now, the project only supports OpenGL, but in theory, any graphical
backend that implements the Rust-Graphics backend interface could be used
without any changes to your drawing.

The __Rust-Graphics__ drawing API is based on the pure functional idea of
building more complicated structures out of simpler ones without performing
any mutation.

A code snippit is worth a thousand words, so here's __Rust-Graphics__ in action:

```rust
// Make a drawing context
let c = &Context::abs(window_width, window_height);
// Clear the screen with white
c.rgb(1.0, 1.0, 1.0).draw(backend);
// Draw a red rectangle in the upper left corner of the screen
c.rect(0.0, 0.0,  50.0, 50.0).rgb(1.0, 0.0, 0.0).draw(backend);
```

In the api example, `backend` is a __Rust-Graphics__ graphical backend that
in practice will probably be an OpenGL context.

## Rust-Image

Odds are that your game will have some images as assets to be drawn into the
game.  Fortunately, __Rust-Image__ has you covered, with decoders and encoders
for `png`, `jpeg`, `gif`, and `webp`.  Example code for this library isn't
particularly interesting: it does exactly what you expect exactly how you would
expect.

# Inter-Package Dependencies

You might have noticed that for both __Piston__ and __Rust-Graphics__ there was
a lot of talk about being independent from "backends".  This is because both
projects use interfaces heavily in order to minimize hard dependencies.

For example, __Piston__ has a `GameWindow` trait with _no implementations_.
All of the rest of the code in __Piston__ simply operates on _a_ `GameWindow`.

Because SDL2 and GLFW are great libraries for window and event logic, a game
developer can choose either one and depend on the interface implementation
that the Piston Developers have already written for that choice.

Expanding on that example, a dependency tree might look like this.

* My Game
  * piston
  * graphics
  * sdl2_game_window
    * sdl2
    * piston
    * gl
  * opengl_graphics
    * gl
    * graphics
    * image

In that project, the developer wanted to use the packages `piston` and
`graphics`.  However, both of those projects are useless without a backend
implementation.  The developer chose `sdl2` for the `piston` backend
and OpenGL for the `graphics` backend.

Now without being exposed to raw SDL2 or OpenGL, the programmer can use
the well idiomatically designed __Piston__ and __rust-graphics__ libraries.
