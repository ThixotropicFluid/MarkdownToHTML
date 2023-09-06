# HTML staic site generator

This is a create designed to take .md files and convert them into HTML files.

The scope of this tool is limited due to my incopitence.

This tool supports the following features

- Headings
- Linking
- Arbtary HTML injection
- More to come, but this seems like a good starting point for myself

# Usage 
To create the executable
```$ cargo build --release```
The executable can be found in the target/release folder

To run this tool run the executable with a path to a folder with a .jaml files. The tool will parse the files into .html files by appenging preamble.txt and postamble.txt. More complex html files can be added manually by adding .html files to the root directory.

# .jaml syntax
Newlines are directed 1 to 1 from the .jaml file to .html

All other features use '\' as an escape key
- Headings 1-6 are suppored with hashes (\# Heading level 1)
- Linking to external and internal pages (\[display](https://github.com))
- Arbitrary HTML (\\{\<p\> arbitrary \<b\>HTML\</b\> content is contained here\</p\>})
