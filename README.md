<div align="center">
  <img width="160" src="./src-tauri/icons/icon.png" />
  <h1>PathLinker</h1>
  <p>🔗 Link any file anywhere on your computer! 🔗</p>
</div>

[![English badge](https://img.shields.io/badge/%E8%8B%B1%E6%96%87-English-blue)](./README.md)
[![简体中文 badge](https://img.shields.io/badge/%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87-Simplified%20Chinese-blue)](./README_CN.md)

## ✨ Features
- Supports Windows, Linux
- Allows you to convert the path of your files into URLs through the program, which can be used in places where URLs are supported, such as markdown, HTML, and other programs that allow URL insertion
- Files corresponding to URLs can be opened via the URL

## 📦 Installation
### Windows
1. Download and unzip [pathlinker_0.1.0.1_x64_windows.zip](https://github.com/JeseKi/PathLinker/releases/download/preview/pathlinker_0.1.0.1_x64_windows.zip)
2. Double-click `pathlinker_0.1.0.1_x64_windows.msi` to install (**recommended not to change the installation path**)
3. Run `register.bat` to add registry entries.

### Linux
#### deb
1. Download the [deb package](https://github.com/JeseKi/PathLinker/releases/download/preview/pathlinker_linux_0.1.0-1_amd64_AppImage.zip)

2. Execute `sudo dpgk -i pathlinker_linux_0.1.0-1_amd64.deb`
#### AppImage
1. Download and unzip [pathlinker_linux_0.1.0-1_amd64_AppImage.zip](https://github.com/JeseKi/PathLinker/releases/download/preview/pathlinker_linux_0.1.0-1_amd64_AppImage.zip)
2. Execute
```bash
# Add the application to the registry
sudo bash init.sh
# Grant execution permission
sudo chmod +x pathlinker_0.1.0-1_amd64.AppImage
```

### Mac
Since I don't have a Mac yet, I hope developers with Mac can help me complete this part. If interested, please contact my email.

## 🛠️ Usage
1. Open `pathlinker`
2. Click `select files`
![step2](README/step2.png)
3. Select the file(s) you want to map (you can select multiple at once)
![step3](README/step3.png)
4. You can click `copy` to copy the URL corresponding to the mapping
![step4](README/step4.png)
5. Now you can use the URL to open the file in other places, such as Obsidian (a Markdown note-taking software):
![open_1](README/open_1.png)

Or XMind (a mind mapping software):
![open_2](README/open_2.png)
![open_3](README/open_3.png)

And even HTML:

![open_4](README/open_4.png)

6. When you click on the URL, the file will be opened with the default software:
![open_5](README/open_5.png)

## 📝 ToDo
- [ ] Support for Mac
- [ ] Multilingual support
- [ ] Automatic updates
- [ ] Settings page
  - [ ] Specifically, allow for backup and automatic updates for some/all mapped files
  - [ ] Automatically track changes in the path of monitored files in a specified directory and update the database automatically
  - [ ] Use specific software to open files with specified extensions.
- [ ] Implementation of error reporting when opening files, instead of showing `log.txt`
- [ ] Allow for exporting/importing file mapping data

## 😕 FAQ

### Why didn't any file open when I clicked on the URL?
Please check if you have followed the steps. On Windows, make sure to run `register.bat`, and on Linux, if you are using AppImage, you need to run `init.sh`. If it still doesn't work, please try submitting issues.

### Is it safe?
PathLinker works by storing the path of a file in a database and generating a random URL to store in the database along with it. When the URL is clicked, the system registry will pass the URL to the application, which will then query the corresponding path in the database and call the default program to open the file based on the obtained path.
Therefore, you don't have to worry about the following issues:
1. Will the file be accessed if the URL is leaked online?
2. Will the file be accessed if the database is leaked online?
3. Will the software accidentally modify important system files?

However, this also means that if you switch to a different computer, you might not be able to open the file via the URL because the database stores the path on your local machine, so you need to remap.

We will address this issue with file backup and automatic updates in a future update.