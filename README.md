__Zippy__ is an opensource, cross-platform http server for static files.

## Looking for latest release of a version branch?

| __Zippy versions__ | __Status__ | __Release Date__ | __Releases__ |
|--------------------|------------|------------------|--------------|
| v1.0.0 | LTS - Recommended for most users | 2023-12-15 | [Releases](./releases.md) |

To get all previous releases, go to [Releases](./releases.md)

## How to setup zippy

### Linux
* Download the latest linux release
* extract the `.tar.gz` folder, rename the folder to `zippy` move the folder to `~`

```bash
tar -xf zippy.tar.gz && mv zippy ~
```
* open `.bashrc` and edith the `PATH` variable

```bash
cd ~ && nano .bashrc
```
* edit `.bashrc`
```bash
export PATH=$PATH:/home/<Your-Username>/zippy/bin
```
Replace `<Your-Username>` with your username.


### Windows
* Download the latest windows release.

* extract the `.zip` folder, rename the folder to `zippy` move the folder to `C:/Program Files`.

* copy the path to zippy's bin folder, that is `C:/Program Files/zippy/bin`.

* Open `Environment variables` by searching on the `Start` menu.

* On user variables, under `Path`, press `New` then paste `C:/Program Files/zippy/bin`.

* Press apply then ok.


### Testing
Testing if zippy is configured correctly. Open `cmd` then type `zippy --help` press enter.
You will get 
```bash
A simple http server for static files.

Usage: zippy [OPTIONS] [COMMAND]

Commands:
  serve  Serves a specific folder
  help   Print this message or the help of the given subcommand(s)

Options:
  -r, --root <PATH>  Path to the folder you want to serve
  -h, --help         Print help
  -V, --version      Print version
```
This means it's correctly installed and configure.


## Usage 
Use it to serve static files from a folder, example serving a website locally.
```bash
zippy serve /path-to-static-folder
```
Example: to serve your current directory.
```bash
zippy serve ./
```
zippy will look for the root `index.html` and serve it or it will provide a list of files and folders in that directory you are serving if it doesn't find `index.html`.
