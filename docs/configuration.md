# Configuration

Configuration parameters:

- `docs_folder` - a path to a folder which will contain all generated documents. It's an
optional parameter so if you won't set it up all documents will be placed in `docs` folder in
the working directory.
> **NOTE** be careful, all files in the `docs_folder` will be replaced by documentation files.

- `project_path` - an entry point for the parser

- `files_pattern` - unix style pathname pattern for matching files which will be parsed

Fundoc will read all the configuration parameters from the `fundoc.json` config file
which should be placed into the working directory of the programm's proccess (generally, it's a root of a
poject)

You can diable parsing for a part of your file or a whole file by adding this comment: `fundoc-disable`.
If you wan't to turn fundoc on few lines below just add this comment: `fundoc-enable`.
In case when you don't write the enable-comment all text from disable comment until the end of
the file will be ignored
