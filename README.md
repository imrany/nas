__Zippy__ is an opensource, cross-platform http server for static files.

## Looking for latest release of a version branch?

| __Versions__ | __Status__ | __Release Date__ | __Platforms__ | __Releases__ |
|----------|--------|------|---------|----------|
|zippy v0.0.3  | current - Latest features | 2023-12-01 | Windows_x64 | [zippy-v0.0.3-windows-x64.zip](https://github.com/imrany/zippy/releases/download/v0.0.3/windows-x64.zip) |
|zippy v0.0.3  | LTS - Recommended for most users | 2023-11-28 | Linux_x64 | [zippy-v0.0.3-linux-x64.tar.gz](https://github.com/imrany/zippy/releases/download/v0.0.3/linux-x64.tar.gz) |
|zippy v0.0.2  | Not maintained - unstable | 2023-11-23 | Linux_x64 | [zippy-v0.0.2-linux-x64.tar.gz](https://github.com/imrany/zippy/releases/download/v0.0.2/zippy.tar.gz) |

To get all previous releases, go to [Downloads](./downloads.md)

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
Testing if zippy is configured correctly. Open `cmd` then type `zippy` press enter.
You will get 
```bash
Problem parsing arguments: Enter path 
example: zippy /home/username/Desktop/static
```
This means it's correctly installed and configure.


## Usage 
Use it to serve static files from a folder, example serving a website locally.
```bash
zippy /path-to-static-folder
```
Example: to serve your current directory.
```bash
zippy ./
```
zippy will look for the root `index.html` and serve it or it will provide a list of files and folders in that directory you are serving if it doesn't find `index.html`.
