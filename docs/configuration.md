# Configuration

Configuration parameters:
[[~]](https://github.com/daynin/fundoc/blob/master/src/config.rs#L6-L8)

- `docs_folder` - a path to a folder which will contain all generated documents. It's an
optional parameter so if you won't set it up all documents will be placed in `docs` folder in
the working directory.

> **NOTE** be careful, all files in the `docs_folder` will be replaced by documentation files.
[[~]](https://github.com/daynin/fundoc/blob/master/src/config.rs#L13-L19)

- `project_path` - an entry point for the parser
[[~]](https://github.com/daynin/fundoc/blob/master/src/config.rs#L23-L25)

- `files_patterns` - unix style pathname patterns for matching files which will be parsed.
[[~]](https://github.com/daynin/fundoc/blob/master/src/config.rs#L29-L31)

- `repository_host` - an http url which will be used for creating a link to a file in a
repository. For example, if you want to add links to your files for each section you can pass
a value like `https://github.com/user_name/project_name/blob/master`. It will be used for
creating an url like this
`https://github.com/user_name/project_name/blob/master/path/to/your/file.txt`.
[[~]](https://github.com/daynin/fundoc/blob/master/src/config.rs#L35-L41)

- `comment_start_string` - a string which marks the start of a comments block. Example: &#47;\*\*
- `comment_prefix` - a comment line prefix. Example: \*
- `comment_end_string` - a string which marks the end of a comments block. Example: \*&#47;
[[~]](https://github.com/daynin/fundoc/blob/master/src/config.rs#L45-L49)

Fundoc will read all the configuration parameters from the `fundoc.json` config file
which should be placed into the working directory of the programm's proccess (generally, it's a root of a
poject)
[[~]](https://github.com/daynin/fundoc/blob/master/src/config.rs#L57-L61)

You can diable parsing for a part of your file or a whole file by adding this comment: `fundoc-disable`.
If you wan't to turn fundoc on few lines below just add this comment: `fundoc-enable`.

In case when you don't write the enable-comment all text from disable comment until the end of
the file will be ignored
[[~]](https://github.com/daynin/fundoc/blob/master/src/parser.rs#L65-L71)
