__Anvel__ is an opensource, cross-platform http server for static files.

## Looking for latest release of a version branch?

| __Anvel versions__ | __Status__ | __Release Date__ | __Releases__ |
|--------------------|------------|------------------|--------------|
| v1.0.2 | LTS - Recommended for most users | 2023-12-25 | [Releases](./releases.md) |
| v1.0.1 | Not Maintained - Obsolete | 2023-12-16 | [Releases](./releases.md) |
| v1.0.0 | Not maintained - Obsolete | 2023-12-15 | [Releases](./releases.md) |

To get all previous releases, go to [Releases](./releases.md)

## How to setup Anvel

### Linux
* Download the latest linux release
* extract the `.tar.gz` folder, rename the folder to `anvel` move the folder to `~`

```bash
tar -xf anvel.tar.gz && mv anvel ~
```
* open `.bashrc` and edith the `PATH` variable

```bash
cd ~ && nano .bashrc
```
* edit `.bashrc`
```bash
export PATH=$PATH:/home/<Your-Username>/anvel/bin
```
Replace `<Your-Username>` with your username.


### Windows
* Download the latest windows release.

* extract the `.zip` folder, rename the folder to `anvel` move the folder to `C:/Program Files`.

* copy the path to anvel's bin folder, that is `C:/Program Files/anvel/bin`.

* Open `Environment variables` by searching on the `Start` menu.

* On user variables, under `Path`, press `New` then paste `C:/Program Files/anvel/bin`.

* Press apply then ok.


### Testing
Testing if anvel is configured correctly. Open `cmd` then type `anvel --help` press enter.
You will get 
```bash
A simple http server for static files.

Usage: anvel [OPTIONS] [COMMAND]

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
anvel serve /path-to-static-folder
```
Example: to serve your current directory.
```bash
anvel serve ./
```
anvel will look for the root `index.html` and serve it or it will provide a list of files and folders in that directory you are serving if it doesn't find `index.html`.
