@echo off
:: Check for privileges
net session >nul 2>&1
if %errorlevel% == 0 (
    echo Administrator privileges confirmed, continuing script execution...
) else (
    echo Requesting administrator privileges...
    :: Rerun the script with administrator privileges
    >nul 2>&1 "%SYSTEMROOT%\System32\cacls.exe" "%SYSTEMROOT%\System32\config\system"

    echo Creating a temporary VBScript to elevate privileges...
    :: Create a temporary VBScript to elevate the privileges
    echo Set UAC = CreateObject^("Shell.Application"^) > "%temp%\getadmin.vbs"
    echo UAC.ShellExecute "%~s0", "", "", "runas", 1 >> "%temp%\getadmin.vbs"

    echo Running the VBScript to request elevated permissions...
    :: Execute the VBScript
    "%temp%\getadmin.vbs"
    del "%temp%\getadmin.vbs"
    exit
)

echo Administrator privileges have been successfully confirmed.
SET KEY_NAME="HKEY_CLASSES_ROOT\pathlinker"
SET VALUE_NAME="URL Protocol"

echo Adding registry key...
REG ADD %KEY_NAME% /f
echo Registry key added successfully.

echo Setting URL protocol value...
REG ADD %KEY_NAME% /v %VALUE_NAME% /t REG_SZ /d "" /f
echo URL protocol value set successfully.

echo Setting command registry value...
REG ADD %KEY_NAME%\shell\open\command /ve /t REG_SZ /d "\"C:\Program Files\pathlinker\pathlinker.exe\" \"%%1\"" /f
echo Command registry value set successfully.

echo Protocol registered successfully.
pause
