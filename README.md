# rtr
```rtr``` is a command-line tool for text processing.

## Usage
```bash
rtr program [file]
```
```program``` is a mandatory argument that specifies a program that will be used for text processing.

```file``` is an optional argument that specifies a filename to read the input from. If it's omitted then the input will be read from ```stdin```.

## Features
### GridSlice
Provides a way to perform text filtering and transformation based on a simple python-like grid indexing.
All the text could be treated like a **grid** where each line represents a row and each word in the line represents a column. For example this text:
```bash
-rw-r--r-- 1 4rtzel 4rtzel  134 May  6 11:24 Cargo.lock
-rw-r--r-- 1 4rtzel 4rtzel  212 May  6 11:24 Cargo.toml
drwxr-xr-x 7 4rtzel 4rtzel 4.0K May  6 11:42 .git
-rw-r--r-- 1 4rtzel 4rtzel    8 May  6 11:24 .gitignore
-rw-r--r-- 1 4rtzel 4rtzel 1.1K May  6 11:24 LICENSE
drwxr-xr-x 3 4rtzel 4rtzel 4.0K May  6 11:24 src
drwxr-xr-x 3 4rtzel 4rtzel 4.0K Apr 29 16:41 target
```

could be presented in the following grid:

| - | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
| - | - | - | - | - | - | - | - | - | - |
| 0 | -rw-r--r-- | 1 | 4rtzel | 4rtzel |  134 | May |  6 | 11:24 | Cargo.lock
| 1 | -rw-r--r-- | 1 | 4rtzel | 4rtzel |  212 | May |  6 | 11:24 | Cargo.toml
| 2 | drwxr-xr-x | 7 | 4rtzel | 4rtzel | 4.0K | May |  6 | 11:42 | .git
| 3 | -rw-r--r-- | 1 | 4rtzel | 4rtzel |    8 | May |  6 | 11:24 | .gitignore
| 4 | -rw-r--r-- | 1 | 4rtzel | 4rtzel | 1.1K | May |  6 | 11:24 | LICENSE
| 5 | drwxr-xr-x | 3 | 4rtzel | 4rtzel | 4.0K | May |  6 | 11:24 | src
| 6 | drwxr-xr-x | 3 | 4rtzel | 4rtzel | 4.0K | Apr | 29 | 16:41 | target

We then could use a python-like indexing to extract the text that we want. The indexing syntax looks like that:
```bash
l<line-from>f<field-from>:l<line-to>f<field-to>:l<line-step>f<field-step>
```

* ```l<line-from>``` -- first line to extract text from (e.g. ```l20``` will extract lines from line 20 onward).
* ```f<field-from>``` -- first field (aka word) to extract text from (e.g. ```f3``` will extract words from word 3 onward).
* ```l<line-to>``` -- last line to extract (e.g. ```l30``` will extract all lines before line 31).
* ```f<field-to>``` -- last field (aka word) to extract (e.g. ```f7``` will extract all words before word 8).
* ```l<line-step>``` -- step to use for line extraction (e.g. ```l2``` will extract every second line).
* ```f<field-step>``` -- step to use for field (aka word) extraction (e.g. ```f3``` will extract every third word).

```from``` and ```to``` could have negative values. In that case, the actual number will be calculated by subtracting this value from the input's length 
(e.g. ```l-5``` means the fifth line from the end).

```step``` could also be negative (but not 0). In that case, the output will be reversed 
(e.g. ```f-2``` for step means to reverse the output and extract every second word).


All these indexes have default values and thus could be omitted. The default values are the following:

```bash
l0f0:l-1f-1:l1f1
```

or if we put it into words:

Extract lines starting from line 0 (```l0```) till the last line (```l-1```) with the step 1 (```l1```).
For all these lines extract words starting from word 0 (```f0```) till the last word (```f-1```) with the step 1 (```f1```).

```from``` and ```to``` values also have a shortcut when you want to specify the same value for both of them.
For example, if you want to extract only the fifth word from each line you'd write ```f5:f5```.
To avoid repeating yourself you could use a capital ```F``` to assign the same value for ```from``` and ```to```: ```F5```.

Note that it's not allowed to use lowercase ```l/f``` with uppercase ```L/F``` in a single range because it would be ambiguous.

### Examples
We'll be using the following input for all examples belove:
```bash
$ ll /proc | tail -20
dr-xr-xr-x  5 root             root                0 May  6 12:54 pressure
-r--r--r--  1 root             root                0 May  6 12:54 sched_debug
-r--r--r--  1 root             root                0 May  6 12:54 schedstat
dr-xr-xr-x  4 root             root                0 May  6 12:54 scsi
lrwxrwxrwx  1 root             root                0 Mar 29 14:00 self -> 2658052
-r--------  1 root             root                0 May  6 12:54 slabinfo
-r--r--r--  1 root             root                0 May  6 12:18 softirqs
-r--r--r--  1 root             root                0 May  6 12:18 stat
-r--r--r--  1 root             root                0 Mar 29 14:00 swaps
dr-xr-xr-x  1 root             root                0 Mar 29 14:00 sys
--w-------  1 root             root                0 May  6 12:54 sysrq-trigger
dr-xr-xr-x  5 root             root                0 May  6 12:54 sysvipc
lrwxrwxrwx  1 root             root                0 Mar 29 14:00 thread-self -> 2658052/task/2658052
-r--------  1 root             root                0 May  6 12:54 timer_list
dr-xr-xr-x  6 root             root                0 May  6 12:54 tty
-r--r--r--  1 root             root                0 May  6 12:18 uptime
-r--r--r--  1 root             root                0 May  6 12:54 version
-r--------  1 root             root                0 May  6 12:54 vmallocinfo
-r--r--r--  1 root             root                0 May  6 12:54 vmstat
-r--r--r--  1 root             root                0 May  6 12:54 zoneinfo
```

in the following way:
```bash
ll /proc | tail -20 | rtr <program>
```
---
Print all lines from line 15:
```bash
$ ll /proc | tail -20 | rtr l15
-r--r--r-- 1 root root 0 May 6 12:18 uptime
-r--r--r-- 1 root root 0 May 6 12:54 version
-r-------- 1 root root 0 May 6 12:54 vmallocinfo
-r--r--r-- 1 root root 0 May 6 12:54 vmstat
-r--r--r-- 1 root root 0 May 6 12:54 zoneinfo
```

Print first 3 lines (indexes start at 0):
```bash
$ ll /proc | tail -20 | rtr :l2
dr-xr-xr-x 5 root root 0 May 6 12:54 pressure
-r--r--r-- 1 root root 0 May 6 12:54 sched_debug
-r--r--r-- 1 root root 0 May 6 12:54 schedstat
```

Print lines from 10 to 12:
```bash
$ ll /proc | tail -20 | rtr l10:l12
--w------- 1 root root 0 May 6 12:54 sysrq-trigger
dr-xr-xr-x 5 root root 0 May 6 12:54 sysvipc
lrwxrwxrwx 1 root root 0 Mar 29 14:00 thread-self -> 2658052/task/2658052
```

Print last 3 lines:
```bash
$ ll /proc | tail -20 | rtr l-3
-r-------- 1 root root 0 May 6 12:54 vmallocinfo
-r--r--r-- 1 root root 0 May 6 12:54 vmstat
-r--r--r-- 1 root root 0 May 6 12:54 zoneinfo
```

From the last 5 lines print the first 2:
```bash
$ ll /proc | tail -20 | rtr l-5:l-4
-r--r--r-- 1 root root 0 May 6 12:18 uptime
-r--r--r-- 1 root root 0 May 6 12:54 version
```

Print only line 10:
```bash
$ ll /proc | tail -20 | rtr L10
--w------- 1 root root 0 May 6 12:54 sysrq-trigger
```

Print every 4th line:
```bash
$ ll /proc | tail -20 | rtr ::l4
dr-xr-xr-x 5 root root 0 May 6 12:54 pressure
lrwxrwxrwx 1 root root 0 Mar 29 14:00 self -> 2658052
-r--r--r-- 1 root root 0 Mar 29 14:00 swaps
lrwxrwxrwx 1 root root 0 Mar 29 14:00 thread-self -> 2658052/task/2658052
-r--r--r-- 1 root root 0 May 6 12:54 version
```

Print lines from 10 to 15 with the step of 2 reversed:
```bash
$ ll /proc | tail -20 | rtr l10:l15:l-2
-r--r--r-- 1 root root 0 May 6 12:18 uptime
-r-------- 1 root root 0 May 6 12:54 timer_list
dr-xr-xr-x 5 root root 0 May 6 12:54 sysvipc
```

Print only the last word in each line:
```bash
$ ll /proc | tail -20 | rtr F-1
pressure
sched_debug
schedstat
scsi
2658052
slabinfo
softirqs
stat
swaps
sys
sysrq-trigger
sysvipc
2658052/task/2658052
timer_list
tty
uptime
version
vmallocinfo
vmstat
zoneinfo
```

Print last 3 lines but the words are reversed:
```bash
$ ll /proc | tail -20 | rtr l-3::f-1
vmallocinfo 12:54 6 May 0 root root 1 -r--------
vmstat 12:54 6 May 0 root root 1 -r--r--r--
zoneinfo 12:54 6 May 0 root root 1 -r--r--r--
```