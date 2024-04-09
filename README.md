# Sycamore Template Generator
[![Crates.io](https://img.shields.io/crates/v/sycamore-template)](https://crates.io/crates/sycamore-template)

Initiate a [sycamore](https://sycamore-rs.netlify.app/) project

## Installation
```
cargo install sycamore-template
```

## Use
This command generates a sycamore project named "ex-project", as in the `ex-project` directory:
```
sycamore-template --name=ex-project
```

</br>

Below command generates a sycamore project with **sycamore-router** implemented. Also the home index will have head lines of copy-dir and css. The example result is in the `ex-router-project` directory.
```
sycamore-template
  --name=ex-router-project
  --router=true
  --copy-dir="/assets"
  --css="/assets/style.css"
```

## Options

full example:
```
sycamore-template
  --name=project-name
  --router=false
  --copy-dir="/assets"
  --css="/style.css"
  --favicon="/favicon.svg"
  --version="0.9.0-beta.2"
```

* name
  - project's name. This is required.
* router
  - bool. default: false.
  - Whether or not to use sycamore-router
* copy-dir
  - Optional. Add a head line in the home `index.html` file with given href:
  - `<link data-trunk rel="copy-dir" href="{copy-dir}"/>`
  - This does not make an actual directory.
* css
  - Optional. Add a head line in the `index.html` file with given href:
  - `<link data-trunk rel="css" href="{x}"/>`
  - This does not make an actual file.
* favicon
  - Optional. Add a favicon head line in the `index.html` with given href:
  - `<link rel="icon" type="image/x-icon" href="{x}">`
* version
  - Optional. default is "0.9.0-beta.2"
  - It would be better to add any crates with their latest version, however, as currently Sycamore has beta version as its latest one, I included this option to manually designate the version. This version value is applied to both of sycamore and sycamore-router.