@echo off
nasm -fwin32 intermediate/inter.bi -o intermediate/inter.obj
golink /fo output\main.exe /ni /console intermediate/inter.obj kernel32.dll
cd output
main.exe
cd ..\
