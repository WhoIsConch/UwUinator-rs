# Welcome to the UwUinator

The UwUinator is a CLI program that fills a directory with what we call "UwU files." 
By default, this is a PNG image titled "uwu.png." In reality, this file can be whatever you want it to, and can be specified in the program. 

## Use the UwUinator

```bash
uwurs.exe [path] [options] 
```

### Options
`--file` - The file to be copied into the directory. Default is "uwu.png"

`--amount` - The amount of files to be copied into the directory. Default is as many as it takes to fill the directory.

`--storage` - The amount of storage the UwUinator should take up.

### Example
Fill a specified path with 1000 UwU files of your choice:
```bash
uwurs.exe D:\path\to\fill --file path\to\my\file.txt --amount 1000
```

Fill a specified path with 10GB of UwU files:
```bash
uwurs.exe D:\path\to\fill --storage 1000000000
```

Completely fill a directory with as many UwU files as can fit:
```bash
uwurs.exe D:\path\to\fill
```

## More Information

The UwUinator \[Pronounced oo-woo-inator\] started as a silly Python script that I wrote during school. 
Now, I try to write an UwUinator in any programming language that I'm trying to learn, as it 
seems like a good sort of introduction to command line arguments and File I/O operations.

Be sure to check out the [original Python-based UwUinator](https://github.com/WhoIsConch/UwUinator)
