⚠️ ***This crate got DEPRECATED: Won't be maintained anymore***

This crate was a personal project developed independently from the official Sycamore versions. And it's not going to be updated or maintained. For information on using Sycamore framework, refer to its official site.

<br/>

# Sycamore Template Generator
[![Crates.io](https://img.shields.io/crates/v/sycamore-template)](https://crates.io/crates/sycamore-template)

Initiate a [sycamore](https://sycamore-rs.netlify.app/) project (since sycamore version of 0.9)

## Installation
```
cargo install sycamore-template
```

## Use

* Simple example: this command generates a sycamore project named "ex-project" (see directory `ex-project`)
```
sycamore-template --name=ex-project
```

* Full example: this command generates a sycamore project named "ex-router-project", which using `sycamore-router`. Also the `index.html` will have head lines of `copy-dir` and `css`. (see directory `ex-router-project`)
```
sycamore-template
  --name=ex-router-project
  --router=true
  --copy-dir="/assets"
  --css="/assets/style.css"
```

## Options
```
sycamore-template
  --name=project-name
  --router=false
  --copy-dir="/assets"
  --css="/style.css"
  --favicon="/favicon.svg"
```

* name
  - project's name. This argument is required.
* router
  - bool. default: false.
  - Whether or not to use `sycamore-router`
* copy-dir
  - Optional. Add a head line in the `index.html` file with given value:
  - `<link data-trunk rel="copy-dir" href="{value}"/>`
  - This does not make an actual directory.
* css
  - Optional. Add a head line in the `index.html` file with given value:
  - `<link data-trunk rel="css" href="{value}"/>`
  - This does not make an actual file.
* favicon
  - Optional. Add a favicon head line in the `index.html` with given value:
  - `<link rel="icon" type="image/x-icon" href="{value}">`
* version:
  - At this moment, sycamore version `0.9` is used for default;


# Logs
```yaml
* `v0.1.0-beta.4`
  - follow up of sycamore v0.9.0-beta.4.
* `v0.1.1`
  - stablized with sycamore v0.9
* `v0.1.2`
  - added `<body></body>` for `index.html`
```