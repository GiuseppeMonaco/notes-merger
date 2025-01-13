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

The merged notes will have the following name:
```
\path\to\notes\out\[NOTE_NAME].png
```

For a guide on how the program works run:
```
note-merger --help
```

### Example
Assume you have the following files (common pattern if exporting from the Samsung Notes app):
```
some_folder/
├─ Course 1 - Lesson 1_241217_130305_1.png
├─ Course 1 - Lesson 1_241217_130305_2.png
├─ Course 1 - Lesson 1_241217_130305_3.png
├─ Course 1 - Lesson 3_241219_175104_1.png
├─ Course 1 - Lesson 3_241219_175409_2.png
├─ subfolder/
│  ├─ Course 1 - Lesson 2_241218_090305_1.png
│  ├─ Course 1 - Lesson 2_241218_090600_2.png
```
Where subdirectory is not necessary but allowed. Merged notes will be saved as follows:
```
some_folder/
├─ ...
├─ out/
│  ├─ Course 1 - Lesson 1.png
│  ├─ Course 1 - Lesson 2.png
│  ├─ Course 1 - Lesson 3.png
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