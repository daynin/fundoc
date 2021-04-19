# Configuration

Configuration parameters:
[[~]](https://github.com/daynin/fundoc/blob/master/src/config.rs#L9-L11)

- `docs_folder` - a path to a folder which will contain all generated documents.
    
     >*NOTE** be careful, all files in the `docs_folder` will be replaced by documentation files.
[[~]](https://github.com/daynin/fundoc/blob/master/src/config.rs#L16-L20)

- `project_path` - an entry point for the parser
[[~]](https://github.com/daynin/fundoc/blob/master/src/config.rs#L24-L26)

- `files_patterns` - unix style pathname patterns for matching files which will be parsed.
[[~]](https://github.com/daynin/fundoc/blob/master/src/config.rs#L30-L32)

- `repository_host` - an http url which will be used for creating a link to a file in a
     repository. For example, if you want to add links to your files for each section you can pass
     a value like `https://github.com/user_name/project_name/blob/master`. It will be used for
     creating an url like this
     `https://github.com/user_name/project_name/blob/master/path/to/your/file.txt`.
[[~]](https://github.com/daynin/fundoc/blob/master/src/config.rs#L36-L42)

- `comment_start_string` - a string which marks the start of a comments block. Example: &#47;\*\*
     - `comment_prefix` - a comment line prefix. Example: \*
     - `comment_end_string` - a string which marks the end of a comments block. Example: \*&#47;
[[~]](https://github.com/daynin/fundoc/blob/master/src/config.rs#L46-L50)

`mdbook` - if true generates documentation in format of [mdBook](https://rust-lang.github.io/mdBook/index.html).
     `book_name` - a name of the result book.
     `book_build_dir` - a directory that contains the build result.
[[~]](https://github.com/daynin/fundoc/blob/master/src/config.rs#L56-L60)

Fundoc will read all the configuration parameters from the `fundoc.json` config file
which should be placed into the working directory of the programm's proccess (generally, it's a root of a
poject)
[[~]](https://github.com/daynin/fundoc/blob/master/src/config.rs#L68-L72)

You can diable parsing for a part of your file or a whole file by adding this comment: `fundoc-disable`.
If you wan't to turn fundoc on few lines below just add this comment: `fundoc-enable`.

In case when you don't write the enable-comment all text from disable comment until the end of
the file will be ignored
[[~]](https://github.com/daynin/fundoc/blob/master/src/parser.rs#L69-L75)
