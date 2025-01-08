# Notes Merger
This tool is used to concatenate images of notes that are exported from a note taking app, like the "Samsung Notes" app. It is especially useful if the exported notes have infinitely scrolling pages, but it could also work for notes that are split into pages.

## Usage
For now, this tool works on sets of files that follow the following naming convention:

```
\path\to\notes\[NOTE_NAME]_[STRING].[IMAGE_EXTENSION]
```

Where:
- [NOTE_NAME] must be the same for the images to be merged in a single image;
- [STRING] is a string (ideally a number) used to decide the sorting of images within the note;
- [IMAGE_EXTENSION] can be ".png", ".jpeg" or ".jpg".

For a guide on how the program works run:

```
note-merger --help
```

## Get started

### Using the prebuilt executable (recommended)
Download the latest release from the [release page](https://github.com/GiuseppeMonaco/notes-merger/releases), and put it wherever you want (it's a portable software).

### Building manually from source
```
git clone https://github.com/GiuseppeMonaco/notes-merger.git
cd notes-merger
cargo build --release
```

## Usage
For a guide on how the program works run:
```
note-merger --help
```