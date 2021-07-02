nasm -fwin32 inter.bi -o inter.obj
golink /ni /console inter.obj kernel32.dll
inter.exe
