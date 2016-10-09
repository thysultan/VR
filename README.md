# Virtual Rendering Implementations

React-Style virtual(diff and patch) rendering implementations in different programming Languages.

`langName-intro-to-lang.ext` features a quick tutorial on the specifics of that languages syntax design
which may serve as a good way to learn a new language from the point of view of another through comparing
how the same thing is implemented in the language vs your language of choice. 

~~NOTE: Calls to native api's are prefixed with a comment `// ... calls native api(s)`.~~

### Current Implementations

- Swift
- Javascript

### Implementations Roadmap

- Rust
- GO

Open to pull requests for other Languages listed or not listed in the list of current implementations or roadmap. 
The only contributing quide is to include a `langName-intro-to-lang.ext` file
that gives a quick run down of the languages common syntax design and 
`langName-virtual-render.ext` that houses the implementation.

### What is React-Style?

It is where the UI view is represent virtually as a render snapshot of the current state
such that when the state of the app changes a new render tree is created of the current state of the view and diff'ed against 
the old render tree by reconsiler that traverses the tree to determine the best way to patch the live view.