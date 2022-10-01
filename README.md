<a name="readme-top"></a>

<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
<h3 align="center">cat-loggr-rs</h3>

  <p align="center">
	A simple, lightweight utility for making beautiful logs, based on <a href="https://github.com/Ratismal/cat-loggr">cat-loggr</a>
    <br />
    <a href="https://github.com/Sven65/cat-loggr-rs"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    ·
    <a href="https://github.com/Sven65/cat-loggr-rs/issues">Report Bug</a>
    ·
    <a href="https://github.com/Sven65/cat-loggr-rs/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

[![cat-loggr-rs Screen Shot][product-screenshot]](#)

A simple, lightweight utility for making beautiful logs, based on [cat-loggr](https://github.com/Ratismal/cat-loggr)



<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- GETTING STARTED -->
## Getting Started

To get a local copy up and running follow these simple example steps.

### Prerequisites

This is an example of how to list things you need to use the software and how to install them.

* [Rust](https://www.rust-lang.org/tools/install)
* [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Installation

1. Install from cargo
   ```sh
	cargo install cat-loggr
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage


```rust
use cat_loggr::CatLoggr;

fn main() {
	let logger = CatLoggr::new(None);

	logger.log("This is a log", "info");
}
```

_For more examples, please refer to the [Documentation](https://docs.rs/cat-loggr/)_

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- Default log levels -->

## Default Log Levels

- fatal
- error
- warn
- trace
- init
- info
- verbose
- debug


<!-- ROADMAP -->
## Roadmap

- [ ] Feature parity
    - [x] Custom log levels
    - [x] After hooks
    - [x] Level threshold
    - [ ] Pre-hook
    - [ ] Argument hooks
    - [ ] Meta
      - [ ] Color option
- [ ] Macro for panics
- [ ] Tests

See the [open issues](https://github.com/Sven65/cat-loggr-rs/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Project Link: [https://github.com/Sven65/cat-loggr-rs](https://github.com/Sven65/cat-loggr-rs)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [Best readme template](https://github.com/othneildrew/Best-README-Template)
* [cat-loggr](https://github.com/Ratismal/cat-loggr)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[issues-shield]: https://img.shields.io/github/issues/Sven65/cat-loggr-rs.svg?style=for-the-badge

[issues-url]: https://github.com/Sven65/cat-loggr-rs/issues
[license-shield]: https://img.shields.io/github/license/Sven65/cat-loggr-rs.svg?style=for-the-badge
[license-url]: https://github.com/Sven65/cat-loggr-rs/blob/master/LICENSE.txt
[product-screenshot]: https://raw.githubusercontent.com/Sven65/cat-loggr-rs/master/images/demo.png