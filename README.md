
# Viktor, your personal running outfit assistant

![screen shot 2016-12-16 at 15 05 37](https://cloud.githubusercontent.com/assets/442035/21265305/2a6040ce-c3a1-11e6-9e83-e91cd494a945.png)

## Table of Contents

* [About](#about)
* [Features](#features)
* [Limitations](#limitations)
* [Prerequisites](#prerequisites)
* [FAQ](#faq)
* [Contributing](#contributing)

### About

Viktor is, at its core, a simple wrapper around the ["what to wear?"](http://www.runnersworld.com/what-to-wear) service
of the Runner's World website. What Viktor adds is trying to automate the process as much as possible, by fetching
the current weather via the [Apixu](https://www.apixu.com) weather API.

### Features

* Display of inline images in an iTerm 2 session (Mac OS X Only)
* Recap of current weather conditions & chosen workout
* Auto inference of the current city based on IP (unless a city is given as a parameter).

### Limitations

I'm fully aware that Viktor occupies a weird niche, and more specifically:

* Although the code is written to be multi-platform, the only terminal which can benefit from
the display of the inline images is [iTerm 2](https://www.iterm2.com/documentation-images.html) on Mac OS X.

* The display of inline images inside of a tmux session is horrid; this is a known [limitation of tmux](https://gitlab.com/gnachman/iterm2/issues/3898).
If someone would like to step up and find a solution which uses only vanilla tmux I would be over the moon!

* The algorithm which maps the current weather conditions to the form values to submit to the Runner's World website
could be improved to give more realistic results.

* It's my first foray into a fully-fledged Rust application, so expect some naive code lurking around!

### Prerequisites

In order to use Viktor, you will need an API Key to access the Apixu weather API.
You can create [a free account easily](https://www.apixu.com/signup.aspx). Once done that,
you will need to export the following env var:

```
export APIXU_API_KEY=xxxxxxxxxxx // Your key
```

### FAQ

* Where the name "Viktor" comes from?

It's freely inspired from the character of _Viktor Navorski_ as seen in [The Terminal](https://en.wikipedia.org/wiki/The_Terminal).
The full disclosure is that he was literally running in a terminal, although of a different kind!

### Contributing

Contributions and PRs are extremely encouraged, of course.
