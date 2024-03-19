@echo off
SET KEY_NAME="HKEY_CLASSES_ROOT\pathlinker"
SET VALUE_NAME="URL Protocol"

REG ADD %KEY_NAME% /f
REG ADD %KEY_NAME% /v %VALUE_NAME% /t REG_SZ /d "" /f
REG ADD %KEY_NAME%\shell\open\command /ve /t REG_SZ /d "\"C:\Program Files\pathlinker\pathlinker.exe\" \"%1\"" /f

echo Protocol registered successfully.
pause
