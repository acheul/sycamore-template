# Sycamore Template Generator
[![Crates.io](https://img.shields.io/crates/v/sycamore-template)](https://crates.io/crates/sycamore-template)

Initiate a [sycamore](https://sycamore-rs.netlify.app/) project (since sycamore version of 0.9)

## Installation
```
cargo install sycamore-template --version=0.1.1
```

## Use

* Simple example: this command generates a sycamore project named "ex-project" (see directory `ex-project`)
```
sycamore-template --name=ex-project
```
</br>

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


# Dev Logs
```yaml
* `v0.1.0-beta.4`
  - follow up of sycamore v0.9.0-beta.4.
* `v0.1.1`
  - stablized with sycamore v0.9
```