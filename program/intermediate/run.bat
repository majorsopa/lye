nasm -fwin32 inter.asm -o inter.obj
golink /ni /console inter.obj kernel32.dll
inter.exe