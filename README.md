## Developing your Leptos CSR project

To develop, running

```sh
trunk serve --port 3000 --open
```

will open your app in your default browser at `http://localhost:3000`.


## Deploying your Leptos CSR project

To build for release, use the command

```sh
trunk build --release
```

This will output the files necessary to run your app into the `dist` folder; you can then use any static site host to serve these files.

