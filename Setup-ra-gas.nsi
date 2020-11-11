; Installer definition for  ne4_konfig
; Written by Stefan Müller <co@zzeroo.com>
!makensis "CargoAttributes.nsi"
!system "CargoAttributes.exe"
!system "wine CargoAttributes.exe"
!include "CargoAttributes.txt"
; optional cleanup
!delfile "CargoAttributes.exe"
!delfile "CargoAttributes.txt"

!include "MUI2.nsh"

Unicode true
SetCompressor /SOLID lzma

;--------------------------------
;Configuration
!define ZZ_APP_NAME "RGMS Konfig"
!define ARCH $%ARCH%
!define NAME_SUFFIX "-ra-gas"
!define ICON_NAME_SUFFIX " (interne Version)"

!ifndef OUTFILE
  !define OUTFILE "${CARGO_PKG_NAME}${NAME_SUFFIX}-${CARGO_PKG_VERSION}-windows-${ARCH}-setup.exe"
!endif

OutFile "${OUTFILE}"
Name "${ZZ_APP_NAME}"
Caption "${ZZ_APP_NAME} ${CARGO_PKG_VERSION} ${ARCH} Setup"

;Default installation folder
InstallDir "$PROGRAMFILES\RA-GAS GmbH\${CARGO_PKG_NAME}${NAME_SUFFIX}"

;Get installation folder from registry if available
InstallDirRegKey HKCU "Software\RA-GAS GmbH\${CARGO_PKG_NAME}${NAME_SUFFIX}" ""

;Request application privileges for Windows Vista
RequestExecutionLevel admin

;--------------------------------
;Interface Settings

;!define MUI_ABORTWARNING
!define MUI_ICON "${NSISDIR}\Contrib\Graphics\Icons\nsis3-install.ico"
!define MUI_UNICON "${NSISDIR}\Contrib\Graphics\Icons\nsis3-uninstall.ico"

!define MUI_HEADERIMAGE
!define MUI_HEADERIMAGE_BITMAP "${NSISDIR}\Contrib\Graphics\Header\nsis3-gray.bmp"
!define MUI_WELCOMEFINISHPAGE_BITMAP "${NSISDIR}\Contrib\Graphics\Wizard\nsis3-gray.bmp"

;--------------------------------
;Pages

!insertmacro MUI_PAGE_LICENSE "LICENSE"
!insertmacro MUI_PAGE_COMPONENTS
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

;--------------------------------
;Languages

!insertmacro MUI_LANGUAGE "German"

;--------------------------------
;Installer Sections

Section "Main" SecMain
  SetOutPath "$INSTDIR"

  File /r "${CARGO_PKG_NAME}${NAME_SUFFIX}-${CARGO_PKG_VERSION}-windows-${ARCH}/"

  ;Store installation folder
  WriteRegStr HKCU "Software\RA-GAS GmbH\${CARGO_PKG_NAME}${NAME_SUFFIX}" "" $INSTDIR

  ;Create uninstaller
  WriteUninstaller "$INSTDIR\Uninstall.exe"

  ; Desktop symbols
  CreateShortcut "$DESKTOP\${ZZ_APP_NAME}${ICON_NAME_SUFFIX}.lnk" "$INSTDIR\${CARGO_PKG_NAME}.exe" "" "$INSTDIR\resources\${CARGO_PKG_NAME}${NAME_SUFFIX}.ico" 0

  ; Start menu
  CreateDirectory "$SMPROGRAMS\RA-GAS GmbH"
  CreateShortcut "$SMPROGRAMS\RA-GAS GmbH\Uninstall.lnk" "$INSTDIR\uninstall.exe" "" "$INSTDIR\uninstall.exe" 0
  CreateShortcut "$SMPROGRAMS\RA-GAS GmbH\${CARGO_PKG_NAME}${NAME_SUFFIX}.lnk" "$INSTDIR\${CARGO_PKG_NAME}.exe" "" "$INSTDIR\resources\${CARGO_PKG_NAME}${NAME_SUFFIX}.ico" 0

SectionEnd

;--------------------------------
;Descriptions

  ;Language strings
  LangString DESC_SecMain ${LANG_GERMAN} "Hauptprogramm '${ZZ_APP_NAME}'"

  ;Assign language strings to sections
  !insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
    !insertmacro MUI_DESCRIPTION_TEXT ${SecMain} $(DESC_SecMain)
  !insertmacro MUI_FUNCTION_DESCRIPTION_END

;--------------------------------
;Uninstaller Section

Section "Uninstall"
  Delete "$INSTDIR\resources\*.*"
  RMDIR /r "$INSTDIR\resources"
  Delete "$INSTDIR\share\*.*"
  RMDIR /r "$INSTDIR\share"
  Delete "$INSTDIR\Uninstall.exe"

  ; Remove shortcuts
  Delete "$DESKTOP\${ZZ_APP_NAME}${ICON_NAME_SUFFIX}.lnk"
  Delete "$SMPROGRAMS\RA-GAS GmbH\*.*"

  ; Remove directories used
  RMDir "$SMPROGRAMS\RA-GAS GmbH"

  Delete "$INSTDIR\*.*"
  RMDir "$INSTDIR"

  DeleteRegKey /ifempty HKCU "Software\RA-GAS GmbH\${CARGO_PKG_NAME}${NAME_SUFFIX}"

SectionEnd
