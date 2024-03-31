# newposte-rs

This code just generates a new markdown file with front matter for use with Jekyll.

For example, file `2023-08-07.md`

```yml
---
title: This new post
location: Leicester
tags: [rust, jekyll, blogging]
---
```

And given a media directory with some jpegs (or [jixels](https://github.com/extua/newposte-rs/commit/d44a4dc790307fcb78b47691292160bb0bb1d421)) it'll output a set of picture tags to the file.

```liquid
{% picture media/2023/10/example.jpg --alt an example photo %}
```

