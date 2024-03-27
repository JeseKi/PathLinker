<div align="center">
  <img width="160" src="./src-tauri/icons/icon.png" />
  <h1>PathLinker</h1>
  <p>🔗 在您的计算机上任何地方链接您的任意文件！ 🔗</p>
</div>

[![English badge](https://img.shields.io/badge/%E8%8B%B1%E6%96%87-English-blue)](./README.md)
[![简体中文 badge](https://img.shields.io/badge/%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87-Simplified%20Chinese-blue)](./README_CN.md)

## ✨ 功能
- 支持Windows、Linux
- 可以将您的文件通过程序将路径转为URL，并在可以使用URL的地方使用，比如markdown、HTML、以及一些允许插入URL的其他程序之中
- URL所对应的文件可通过URL进行打开
## 📦 安装
### Windows
1. 下载并解压[pathlinker_0.1.0.1_x64_windows.zip](https://github.com/JeseKi/PathLinker/releases/download/preview/pathlinker_0.1.0.1_x64_windows.zip)
2. 双击`pathlinker_0.1.0.1_x64_windows.msi`进行安装(**推荐不修改安装路径**)
3. 执行`register.bat`来添加注册表项

### Linux
#### deb
1. 下载[deb包](https://github.com/JeseKi/PathLinker/releases/download/preview/pathlinker_linux_0.1.0-1_amd64_AppImage.zip)

2. 执行`sudo dpgk -i pathlinker_linux_0.1.0-1_amd64.deb`
#### AppImage
1. 下载并解压[pathlinker_linux_0.1.0-1_amd64_AppImage.zip](https://github.com/JeseKi/PathLinker/releases/download/preview/pathlinker_linux_0.1.0-1_amd64_AppImage.zip)
2. 执行
```bash
# 将应用添加到注册表
sudo bash init.sh
# 赋予执行权限
sudo chmod +x pathlinker_0.1.0-1_amd64.AppImage
```

### Mac
由于我还没有Mac，因此希望有Mac的开发者能够帮我完成这个部分，如有意向请联系我的邮箱。
## 🛠️ 用法
1. 打开`pathlinker`
2. 点击`select files`
![step2](README/step2.png)
3. 选择您要映射的文件(可以一次性选择多个)
![step3](README/step3.png)
4. 您可以点击`copy`来复制该映射对应的URL
![step4](README/step4.png)
5. 现在您可以在其他地方使用该URL来打开该文件，比如Obsidian(一款Markdown笔记软件):
![open_1](README/open_1.png)

或者XMind(一款思维导图软件):
![open_2](README/open_2.png)
![open_3](README/open_3.png)

当然，也包括HTML:

![open_4](README/open_4.png)

6. 现在您点击了URL后，将会使用默认软件进行打开文件:
![open_5](README/open_5.png)

## 📝 ToDo
- [ ] 支持Mac
- [ ] 多语言
- [ ] 自动更新版本
- [ ] 支持在前端批量管理文件
- [ ] 支持在前端通过文件名搜索已链接的文件
- [ ] 不依赖原始路径
- [ ] 设置页面
  - [ ] 源文件删除时，映射路径自动删除
  - [ ] 设置自动跟踪所使用的目录pathlinker目录
  - [ ] 使用特定的软件来打开指定拓展名的文件
  - [ ] 自定义硬链接创建模式
- [ ] 打开文件时报错的实现，而非显示`log.txt`
- [ ] 允许导出/导入文件映射数据

## 😕 FAQ

### 为什么我点击URL后没有打开任何文件？
请检查是否有按照步骤执行，在Windows上要注意运行`register.bat`，在Linux上如果使用的是AppImage，需要运行`init.sh`。若还是无法运行请尝试提交issues。

### 安全吗？
PathLinker的原理是给文件创建的固定的硬链接，然后将硬链接的路径和存储到数据库中，并生成一个随机的URL一起存储进数据库中，当点击该URL的时候，系统注册表会将URL传递给应用，应用接受到URL后将自动在数据库中查询对应的硬链接路径，根据获取到的路径来调用默认程序打开文件。
因此您可以不用担心下面的问题:
1. URL泄露到网上是否会导致文件被访问？
2. 数据库泄露到网上是否会导致文件被访问？
3. 软件是否会不小心修改系统重要文件？

### 换了台电脑后是否依旧可以正常使用？
是的，换了台电脑后，只需保证数据库文件和文件系统均一起移动就可以保证正常使用。

您可以理解为只要保证硬盘的盘符不变，即使换了电脑也可以正常使用。
对于Windows系统来说，假如您所映射的文件是在C盘，比如这样:`C:\Users\YouName\Desktop\mydoc.docx`，当您换电脑的时候，只需要保证这个文件也在C盘，且也在原本的硬盘上，就可以保证正常使用。
对应Linux系统来说，由于常用我就基本都在home目录下，因此直接将硬盘转移到其他电脑上也可以正常使用。

我们将在后续的更新中增加导出数据库文件和导入数据库文件的功能，以便于进行数据库转移和在不同硬盘、文件系统间进行转移。

### 我所创建的映射是否会额外占用大量硬盘存储空间？
不会。

硬链接的原理是指向一个存储地址，因此一个新的硬链接所占用的硬盘存储空间可以基本忽略不计。

不过由于是创建的硬链接，因此当您删除源文件的时候，源文件本身所占用的硬盘存储空间不会被释放，只有到pathlinker中删除对应的映射后才会释放硬盘存储空间，我们将在后续的更新中增加删除源文件时映射文件也自动删除的设置选项。