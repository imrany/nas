# live_server

__live_server__ is an opensource, cross-platform http server for static files.

## Looking for latest release of a version branch?

| Versions | Date | Platform | Release |
|----------|------|---------|----------|
|live_server v0.0.3  | 2023-12-01 | Windows_x64 | [live_server-v0.0.3-windows-x64.zip](https://github.com/imrany/live_server/releases/download/v0.0.3/windows-x64.zip) |
|live_server v0.0.3  | 2023-11-28 | Linux_x64 | [live_server-v0.0.3-linux-x64.tar.gz](https://github.com/imrany/live_server/releases/download/v0.0.3/linux-x64.tar.gz) |
|live_server v0.0.2  | 2023-11-23 | Linux_x64 | [live_server-v0.0.2-linux-x64.tar.gz](https://github.com/imrany/live_server/releases/download/v0.0.2/live_server.tar.gz) |


## How to setup live_server

### Linux
* Download the latest linux release
* extract the `.tar.gz` folder, rename the folder to `live_server` move the folder to `~`

```bash
tar -xf live_server.tar.gz && mv live_server ~
```
* open `.bashrc` and edith the `PATH` variable

```bash
cd ~ && nano .bashrc
```
* edit `.bashrc`
```bash
export PATH=$PATH:/home/<Your-Username>/live_server/bin
```
Replace `<Your-Username>` with your username.


### Windows
* Download the latest windows release.

* extract the `.zip` folder, rename the folder to `live_server` move the folder to `C:/Program Files`.

* copy the path to live_server's bin folder, that is `C:/Program Files/live_server/bin`.

* Open `Environment variables` by searching on the `Start` menu.

* On user variables, under `Path`, press `New` then paste `C:/Program Files/live_server/bin`.

* Press apply then ok.


### Testing
Testing if live_server is configured correctly. Open `cmd` then type `live_server` press enter.
You will get 
```bash
Problem parsing arguments: Enter path 
example: live_server /home/username/Desktop/static
```
This means it's correctly installed and configure.


## Usage 
Use it to serve static files from a folder, example serving a website locally.
```bash
live_server /path-to-static-folder
```
Example: to serve your current directory.
```bash
live_server ./
```
live_server will look for the root `index.html` and serve it or it will provide a list of files and folders in that directory you are serving if it doesn't find `index.html`.
