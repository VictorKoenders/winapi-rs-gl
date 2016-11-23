// Copyright © 2016 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

pub mod gl;

//! Headers for user mode only

#[cfg(feature = "um-audioclient")] pub mod audioclient;
#[cfg(feature = "um-audiosessiontypes")] pub mod audiosessiontypes;
#[cfg(feature = "um-cfgmgr32")] pub mod cfgmgr32;
#[cfg(feature = "um-cguid")] pub mod cguid;
#[cfg(feature = "um-combaseapi")] pub mod combaseapi;
#[cfg(feature = "um-commctrl")] pub mod commctrl;
#[cfg(feature = "um-commdlg")] pub mod commdlg;
#[cfg(feature = "um-consoleapi")] pub mod consoleapi;
#[cfg(feature = "um-corsym")] pub mod corsym;
#[cfg(feature = "um-d2d1")] pub mod d2d1;
#[cfg(feature = "um-d2dbasetypes")] pub mod d2dbasetypes;
#[cfg(feature = "um-d3d10shader")] pub mod d3d10shader;
#[cfg(feature = "um-d3d11")] pub mod d3d11;
#[cfg(feature = "um-d3d11shader")] pub mod d3d11shader;
#[cfg(feature = "um-d3d12")] pub mod d3d12;
#[cfg(feature = "um-d3d12sdklayers")] pub mod d3d12sdklayers;
#[cfg(feature = "um-d3d12shader")] pub mod d3d12shader;
#[cfg(feature = "um-d3dcommon")] pub mod d3dcommon;
#[cfg(feature = "um-d3dcompiler")] pub mod d3dcompiler;
#[cfg(feature = "um-dbghelp")] pub mod dbghelp;
#[cfg(feature = "um-dcommon")] pub mod dcommon;
#[cfg(feature = "um-docobj")] pub mod docobj;
#[cfg(feature = "um-dpapi")] pub mod dpapi;
#[cfg(feature = "um-dsgetdc")] pub mod dsgetdc;
#[cfg(feature = "um-dsound")] pub mod dsound;
#[cfg(feature = "um-dsrole")] pub mod dsrole;
#[cfg(feature = "um-dwmapi")] pub mod dwmapi;
#[cfg(feature = "um-dwrite")] pub mod dwrite;
#[cfg(feature = "um-errhandlingapi")] pub mod errhandlingapi;
#[cfg(feature = "um-fileapi")] pub mod fileapi;
#[cfg(feature = "um-handleapi")] pub mod handleapi;
#[cfg(feature = "um-heapapi")] pub mod heapapi;
#[cfg(feature = "um-http")] pub mod http;
#[cfg(feature = "um-imm")] pub mod imm;
#[cfg(feature = "um-libloaderapi")] pub mod libloaderapi;
#[cfg(feature = "um-lmaccess")] pub mod lmaccess;
#[cfg(feature = "um-lmdfs")] pub mod lmdfs;
#[cfg(feature = "um-lmerrlog")] pub mod lmerrlog;
#[cfg(feature = "um-lmjoin")] pub mod lmjoin;
#[cfg(feature = "um-lsalookup")] pub mod lsalookup;
#[cfg(feature = "um-memoryapi")] pub mod memoryapi;
#[cfg(feature = "um-minschannel")] pub mod minschannel;
#[cfg(feature = "um-minwinbase")] pub mod minwinbase;
#[cfg(feature = "um-mmdeviceapi")] pub mod mmdeviceapi;
#[cfg(feature = "um-mmsystem")] pub mod mmsystem;
#[cfg(feature = "um-ncrypt")] pub mod ncrypt;
#[cfg(feature = "um-oaidl")] pub mod oaidl;
#[cfg(feature = "um-objidlbase")] pub mod objidlbase;
#[cfg(feature = "um-pdh")] pub mod pdh;
#[cfg(feature = "um-propidl")] pub mod propidl;
#[cfg(feature = "um-propsys")] pub mod propsys;
#[cfg(feature = "um-prsht")] pub mod prsht;
#[cfg(feature = "um-shellapi")] pub mod shellapi;
#[cfg(feature = "um-strmif")] pub mod strmif;
#[cfg(feature = "um-unknwnbase")] pub mod unknwnbase;
#[cfg(feature = "um-vsserror")] pub mod vsserror;
#[cfg(feature = "um-winbase")] pub mod winbase;
#[cfg(feature = "um-wincodec")] pub mod wincodec;
#[cfg(feature = "um-wincon")] pub mod wincon;
#[cfg(feature = "um-wincrypt")] pub mod wincrypt;
#[cfg(feature = "um-winevt")] pub mod winevt;
#[cfg(feature = "um-wingdi")] pub mod wingdi;
#[cfg(feature = "um-wininet")] pub mod wininet;
#[cfg(feature = "um-winineti")] pub mod winineti;
#[cfg(feature = "um-winioctl")] pub mod winioctl;
#[cfg(feature = "um-winnt")] pub mod winnt;
#[cfg(feature = "um-winreg")] pub mod winreg;
#[cfg(feature = "um-winsock2")] pub mod winsock2;
#[cfg(feature = "um-winuser")] pub mod winuser;
#[cfg(feature = "um-xinput")] pub mod xinput;
