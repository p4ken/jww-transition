@REM BVEマップに出力します
@echo off
REM #jww
REM #cd
REM #hm | 自軌道指定 | スキップ |
REM #hc v(VERSION)
REM #:1
REM #h1
REM #hc 自軌道を指定
REM #1  出力始点を指定
REM #g1
REM #:2
REM #h/sub/TRACK-N.bat
REM #e

goto %1

:1
MOVE JWC_TEMP.txt JWC_TEMP_0.txt
goto END

:2
goto END

:END
echo heエラー > JWC_TEMP.txt
