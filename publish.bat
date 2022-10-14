cargo build -Zmultitarget --target x86_64-pc-windows-msvc --target=i686-pc-windows-msvc --release

@RD /S /Q build
mkdir "build"
mkdir "build\x86"
mkdir "build\x64"

echo f | xcopy /f /y "target\i686-pc-windows-msvc\release\soulmemory_rs.dll"   "build\x86\soulmemory_rs.dll"
echo f | xcopy /f /y "target\i686-pc-windows-msvc\release\launcher.exe"        "build\x86\launcher.exe" 
                                                                 
echo f | xcopy /f /y "target\x86_64-pc-windows-msvc\release\soulmemory_rs.dll" "build\x64\soulmemory_rs.dll" 
echo f | xcopy /f /y "target\x86_64-pc-windows-msvc\release\launcher.exe"      "build\x64\launcher.exe" 

pause