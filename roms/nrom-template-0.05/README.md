IF THIS FILE HAS NO LINE BREAKS:  View it in a web browser.  
Windows Notepad is a very popular text editor that comes with the 
Windows operating system, but it doesn't recognize line breaks in 
text files made on Linux or any other UNIX-like operating system.  
Text files with UNIX line breaks display correctly in WordPad, 
Notepad++, Programmer's Notepad, Gedit, or a web browser, just 
not in Windows Notepad.


This is a minimal working program for the Nintendo Entertainment
System using the NROM-128 board.

Concepts illustrated:

* init code
* setting up a static background
* structure of a game loop
* DPCM-safe controller reading
* 8.8 fixed-point arithmetic
* acceleration-based character movement physics
* sprite drawing and animation, with horizontal flipping
* makefile-controlled conversion of sprite sheets

Setting up the build environment
================================

Building this demo requires that the following software be
installed on your computer first:

* ca65 and ld65, the assembly language tools that ship
  with the cc65 C compiler
* Python, a programming language interpreter
* Python Imaging Library, a Python extension to read and write
  bitmap images
* GNU Make, a program to calculate which files need to be
  rebuilt when other files change
* GNU Coreutils, a set of simple command-line utilities for
  file management and text processing

It also requires general familiarity with the command prompt.
You are encouraged to read and understand the articles on general
computer science topics listed at [Before the basics].

[Before the basics]: http://wiki.nesdev.com/w/index.php/Before_the_basics

On Linux
--------

To install Make, Python 2, and Python Imaging Library under Ubuntu:

1. Open a terminal.
2. Type the following, followed by the Enter key:

        sudo apt-get install build-essential python-imaging

3. Type your password to authorize the installation.

To install Make, Python 2, and Python Imaging Library under Fedora:
(instructions suggested by 43110; not tested)

1. Open a terminal and use `su` to become root.
2. Type the following, followed by the Enter key:

        yum install make automake gcc gcc-c++ python python-imaging

Because cc65 is a fairly niche tool, and because the C compiler
portion (which this demo does not use) is not free software as
defined by GNU and Debian, your Linux distribution's default
repository is unlikely to contain it.  This means you will
probably need to install it from source code.

1. Visit [cc65 on GitHub].
2. Click Download ZIP
3. Unzip into a new folder.
4. In a terminal issue the following commands (suggested by 43110):

        cd [path to where you unzipped cc65]
        make
        make install prefix=~/usr

   There's no `./configure` step, and `make install` reportedly
   complained about there being no prefix var. (case sensitive?)

5. Insert the following in your `.bash_profile` or `.bashrc` file,
   to automatically add the local executables to your PATH
   
        if [ -d "$HOME/usr/bin" ] ; then
            PATH="$HOME/usr/bin:$PATH"
        fi

[cc65 on GitHub]: https://github.com/cc65/cc65

On Windows
----------

The MSYS project ports Make, Coreutils, and other key parts of
the GNU operating environment to Windows.  An easy way to install
MSYS is through the automated installer provided by devkitPro.

1. Visit [devkitPro Getting Started].
2. Follow the instructions there to download and run the
   devkitPro Automated Installer.
3. In the installer, check only MSYS.  Don't check devkitARM,
   devkitPPC, devkitPSP, or any of the libraries for newer platforms
   unless you want to start developing for one of those soon.
   
To install Python under Windows:

1. Visit [Python home page].
2. Under Downloads, under Download for Windows, click Python 2.7.8.
   (This step may change if the maintainers of the Python web site
    reorganize the Python web site.)
3. In your web browser's downloads folder, run `python-2.7.8.msi`
4. Follow the prompts through the installer wizard.

To install Python Imaging Library under Windows:

1. Visit [Pillow project page].
2. Under Package, click the latest version.
3. Click Downloads.
4. Click `Pillow-2.5.3.win32-py2.7.exe`. The `py-2.7` part MUST
   match the major and minor version of Python installed on your PC.
   (In Python 2.7.8, the major version is 2, the minor version is 7,
   and the bug fix version is 8.)
5. In your web browser's downloads folder, run
   `Pillow-2.5.3.win32-py2.7.exe`
6. Follow the prompts through the installer wizard

To install cc65 under Windows:

1. Visit [cc65 introduction].
2. Scroll to the bottom.
3. Click "Windows Snapshot" to download a zip file.
4. Open the zip file.
5. Inside the zip file, open the bin folder.
6. Drag `ca65.exe` and `ld65.exe` into a new folder.
7. Add this folder to your "Path" environment variable.

Finally, to make ca65, ld65, and Python available to Make, you'll
need to add the folders containing `ca65.exe`, `ld65.exe`, and
`python.exe` to the `Path` environment variable.  Because the steps
for setting environment variables differ between versions of Windows,
you'll want to search the web for `windows x.x path variable`,
replacing `x.x` with `7`, `8.1`, etc.

Finally, open the makefile in a text editor and change EMU to
whatever NES emulator you have installed.

[devkitPro Getting Started]: http://devkitpro.org/wiki/Getting_Started
[Python home page]: https://www.python.org/
[Pillow project page]: https://pypi.python.org/pypi/Pillow
[cc65 introduction]: http://cc65.github.io/cc65/

Organization of the program
===========================

Include files
-------------

* `nes.inc`: Register definitions and useful macros
* `global.inc`: Global variable and function declarations
* `mmc1.inc`

Source code files
-----------------

* `nrom.s`: iNES header for NROM
* `init.s`: PPU and CPU I/O initialization code
* `main.s`: Main program
* `bg.s`: Background graphics setup
* `player.s`: Player sprite graphics setup and movement
* `pads.s`: Read the controllers in a DPCM-safe manner
* `ppuclear.s`: Useful subroutines for interacting with the S-PPU

Each source code file is made up of subroutines that start with
`.proc` and end with `.endproc`.  See the [ca65 Users Guide] for
what these mean.

[ca65 Users Guide]: http://cc65.github.io/doc/ca65.html

The tools
=========

The `tools` folder contains a couple essential command-line programs
written in Python to convert graphics into a form usable by the NES.
The makefile contains instructions to run the program again whenever
the original asset data changes.

* `pilbmp2nes.py` is a program to convert bitmap images in PNG or
  BMP format into tile data usable by several classic video game
  consoles.  It has several options to control the data format; use
  `pilbmp2nes.py --help` from the command prompt to see them all.

Greets
======

* [NESdev Wiki] and forum contributors
* [FCEUX] team
* Joe Parsell (Memblers) for getting me into NESdev in the first place
* Jeremy Chadwick (koitsu) for more code organization tips

[NESdev Wiki]: http://wiki.nesdev.com/
[FCEUX]: http://fceux.com/

Legal
=====

The demo is distributed under the following license, based on the
GNU All-Permissive License:

> Copyright 2011-2015 Damian Yerrick
> 
> Copying and distribution of this file, with or without
> modification, are permitted in any medium without royalty provided
> the copyright notice and this notice are preserved in all source
> code copies.  This file is offered as-is, without any warranty.

