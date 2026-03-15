; ZipRS NSIS Installer Hooks
; Adds/removes the install directory to/from the user PATH so the CLI
; is accessible from any terminal (e.g.  ziprs list archive.zip)

!macro NSIS_HOOK_POSTINSTALL
  ; Read current user PATH
  ReadRegStr $0 HKCU "Environment" "Path"
  ; If empty, just set it; otherwise append
  StrCmp $0 "" 0 +3
    WriteRegExpandStr HKCU "Environment" "Path" "$INSTDIR"
    Goto _path_done
  ; Check if already present to avoid duplicates
  StrCpy $1 "$0"
  Push "$1"
  Push "$INSTDIR"
  Call StrContains
  Pop $2
  StrCmp $2 "" 0 _path_done
    WriteRegExpandStr HKCU "Environment" "Path" "$0;$INSTDIR"
  _path_done:
  ; Notify all windows that environment has changed
  SendMessage ${HWND_BROADCAST} ${WM_SETTINGCHANGE} 0 "STR:Environment" /TIMEOUT=5000
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
  ; Remove install directory from user PATH using PowerShell
  nsExec::ExecToLog 'powershell -NoProfile -Command "$$p = [Environment]::GetEnvironmentVariable(''Path'',''User''); if($$p){$$parts = $$p -split '';'' | Where-Object { $$_ -ne ''$INSTDIR'' }; [Environment]::SetEnvironmentVariable(''Path'', ($$parts -join '';''), ''User'')}"'
  ; Notify all windows
  SendMessage ${HWND_BROADCAST} ${WM_SETTINGCHANGE} 0 "STR:Environment" /TIMEOUT=5000
!macroend

; Helper function: check if string contains substring
; Usage: Push "haystack" / Push "needle" / Call StrContains / Pop $result
Function StrContains
  Exch $R1 ; needle
  Exch
  Exch $R2 ; haystack
  Push $R3
  Push $R4
  StrLen $R3 $R1
  StrCpy $R4 0
  loop:
    StrCpy $0 $R2 $R3 $R4
    StrCmp $0 "" notfound
    StrCmp $0 $R1 found
    IntOp $R4 $R4 + 1
    Goto loop
  found:
    StrCpy $0 $R1
    Goto done
  notfound:
    StrCpy $0 ""
  done:
  Pop $R4
  Pop $R3
  Pop $R2
  Pop $R1
  Push $0
FunctionEnd
