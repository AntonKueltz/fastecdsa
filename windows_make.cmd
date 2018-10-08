if not exist  aria2c.exe (
	echo "Download aria2c.exe from https://github.com/aria2/aria2/releases/tag/release-1.34.0"
	goto :exit
	)

if not exist  "C:\Program Files (x86)\WinRAR\WinRAR.exe" (
	echo "Need some tool to extract bz2 and zip archives"
	goto :exit
	)
	
if exist "C:\Users\%username%\AppData\Local\Programs\Common\Microsoft\Visual C++ for Python\9.0\VC\bin\cl.exe" (
	goto :getyasm
	) else (
	aria2c.exe https://download.microsoft.com/download/7/9/6/796EF2E4-801B-4FC4-AB28-B59FBF6D907B/VCForPython27.msi
	msiexec.exe /i VCForPython27.msi
	)
	
	
:getyasm
	if not  exist "C:\Users\%username%\AppData\Local\Programs\Common\Microsoft\Visual C++ for Python\9.0\VC\bin\yasm.exe" (
	aria2c.exe http://www.tortall.net/projects/yasm/releases/yasm-1.3.0-win32.exe 
	copy yasm-1.3.0-win32.exe "C:\Users\%username%\AppData\Local\Programs\Common\Microsoft\Visual C++ for Python\9.0\VC\bin\yasm.exe"
	)
	
:getmpir
	if exist "mpir-2.6.0.tar.bz2" (
	goto :make_mpir
	) else (	
	aria2c.exe  http://www.mpir.org/mpir-2.6.0.tar.bz2 
	"C:\Program Files (x86)\WinRAR\WinRAR.exe" x mpir-2.6.0.tar.bz2 mpir-2.6.0
	)

:make_mpir	
	if not exist "C:\Users\User\AppData\Local\Programs\Common\Microsoft\Visual C++ for Python\9.0\VC\lib\gmp.lib" (
	cd mpir-2.6.0\win
	call "C:\Users\%username%\AppData\Local\Programs\Common\Microsoft\Visual C++ for Python\9.0\vcvarsall.bat" 
	call configure.bat ABI 32
	call make.bat 
	REM make check
	call gen_mpir_h.bat
	copy mpir.lib "C:\Users\User\AppData\Local\Programs\Common\Microsoft\Visual C++ for Python\9.0\VC\lib\gmp.lib"
	cd ..
	copy gmp.h "C:\Users\User\AppData\Local\Programs\Common\Microsoft\Visual C++ for Python\9.0\VC\include\gmp.h"
	cd ..
	)

REM have to be replaced to https://github.com/AntonKueltz/fastecdsa/archive/master.zip  if commited	
aria2c.exe  https://github.com/shikuk/fastecdsa/archive/master.zip
"C:\Program Files (x86)\WinRAR\WinRAR.exe" x fastecdsa-master.zip 
cd fastecdsa-master
python setup.py build
python setup.py install

cd..

python -m fastecdsa.test

:exit
exit /b 1




		