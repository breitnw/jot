# Jot

With the sheer number of distractions modern technologies pose, even a task as simple as jotting down a note can lead us down a path of distraction. Sometimes, we even forget why we opened our phone in the first place. Jot aims to address this issue by stripping note-taking down to its barest elements, enabling you to write down your idea as fast as possible and file it away whenever you’re ready.

## What it does

The crux of Jot is its separation into two entirely distinct applications: one for taking notes (the Scratchpad), and one for viewing and filing them (the Inbox). The Scratchpad is as minimalistic as it gets: you have a text box, a priority button, and a Jot button, which uploads your note to the cloud (with a fancy animation, of course). You cannot view past notes, add metadata to notes, or anything else that might distract you from your task: writing down that idea as quickly as possible.

The Inbox is where you address your ideas. It’s meant to be used on your desktop—when you really have time to parse through, refile, or address your thoughts. Notes are sorted by the priority you assigned earlier, so you’ll always see your most important ideas first. When you’re done giving your idea its forever home, you can dismiss it from Jot.

<!-- TODO talk about creating database -->

## Technologies used
Jot runs on Rust. The following libraries were used to create it: 

- [Rocket](https://rocket.rs/), for the API and Inbox backend
- [SDL2](https://crates.io/crates/sdl2), for rendering the Scratchpad
- [Chrono](https://crates.io/crates/chrono), for date and time handling
- [MiniJinja](https://github.com/mitsuhiko/minijinja), for HTML templating
- [Reqwest](https://github.com/seanmonstar/reqwest), for making HTTP requests from the scratchpad
