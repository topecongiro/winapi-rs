// Copyright © 2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::__int64;
use shared::basetsd::SIZE_T;
use shared::minwindef::{BOOL, DWORD, FILETIME, LPCVOID, LPVOID, UCHAR};
use um::wincrypt::ALG_ID;
use um::winnt::{LPCSTR, LPCWSTR};
pub const DELTA_FILE_SIZE_LIMIT: SIZE_T = 32 * 1024 * 1024;
pub const DELTA_OPTIONS_SIZE_LIMIT: SIZE_T = 128 * 1024 * 1024;
UNION!{union DELTA_INPUT_U {
    [usize; 1],
    lpcStart lpcStart_mut: LPCVOID,
    lpStart lpStart_mut: LPVOID,
}}
STRUCT!{struct DELTA_INPUT {
    u: DELTA_INPUT_U,
    uSize: SIZE_T,
    Editable: bool,
}}
pub type LPDELTA_INPUT = *mut DELTA_INPUT;
pub type LPCDELTA_INPUT = *const DELTA_INPUT;
STRUCT!{struct DELTA_OUTPUT {
    lpStart: LPVOID,
    uSize: SIZE_T,
}}
pub type LPDELTA_OUTPUT = *mut DELTA_OUTPUT;
pub type LPCDELTA_OUTPUT = *const DELTA_OUTPUT;
pub type DELTA_FILE_TYPE = __int64;
pub const DELTA_FILE_TYPE_RAW: DELTA_FILE_TYPE = 0x00000001;
pub const DELTA_FILE_TYPE_I386: DELTA_FILE_TYPE = 0x00000002;
pub const DELTA_FILE_TYPE_IA64: DELTA_FILE_TYPE = 0x00000004;
pub const DELTA_FILE_TYPE_AMD64: DELTA_FILE_TYPE = 0x00000008;
pub const DELTA_FILE_TYPE_CLI4_I386: DELTA_FILE_TYPE = 0x00000010;
pub const DELTA_FILE_TYPE_CLI4_AMD64: DELTA_FILE_TYPE = 0x00000020;
pub const DELTA_FILE_TYPE_CLI4_ARM: DELTA_FILE_TYPE = 0x00000040;
pub const DELTA_FILE_TYPE_CLI4_ARM64: DELTA_FILE_TYPE = 0x00000080;
pub const DELTA_FILE_TYPE_SET_RAW_ONLY: DELTA_FILE_TYPE = DELTA_FILE_TYPE_RAW;
pub const DELTA_FILE_TYPE_SET_EXECUTABLES_1: DELTA_FILE_TYPE = DELTA_FILE_TYPE_RAW
                                                             | DELTA_FILE_TYPE_I386
                                                             | DELTA_FILE_TYPE_IA64
                                                             | DELTA_FILE_TYPE_AMD64;
pub const DELTA_FILE_TYPE_SET_EXECUTABLES: DELTA_FILE_TYPE = DELTA_FILE_TYPE_SET_EXECUTABLES_1;
pub const DELTA_FILE_TYPE_SET_EXECUTABLES_2: DELTA_FILE_TYPE = DELTA_FILE_TYPE_RAW
                                                             | DELTA_FILE_TYPE_CLI4_I386
                                                             | DELTA_FILE_TYPE_IA64
                                                             | DELTA_FILE_TYPE_CLI4_AMD64
                                                             | DELTA_FILE_TYPE_CLI4_ARM;
pub const DELTA_FILE_TYPE_SET_EXECUTABLES_3: DELTA_FILE_TYPE = DELTA_FILE_TYPE_RAW
                                                             | DELTA_FILE_TYPE_CLI4_I386
                                                             | DELTA_FILE_TYPE_IA64
                                                             | DELTA_FILE_TYPE_CLI4_AMD64
                                                             | DELTA_FILE_TYPE_CLI4_ARM
                                                             | DELTA_FILE_TYPE_CLI4_ARM64;
pub const DELTA_FILE_TYPE_SET_EXECUTABLES_LATEST : DELTA_FILE_TYPE = DELTA_FILE_TYPE_SET_EXECUTABLES_3;
pub type DELTA_FLAG_TYPE = __int64;
pub const DELTA_FLAG_NONE: DELTA_FLAG_TYPE = 0x00000000;
pub const DELTA_APPLY_FLAG_ALLOW_PA19: DELTA_FLAG_TYPE = 0x00000001;
pub const DELTA_FLAG_E8: DELTA_FLAG_TYPE = 0x00000001;
pub const DELTA_FLAG_MARK: DELTA_FLAG_TYPE = 0x00000002;
pub const DELTA_FLAG_IMPORTS: DELTA_FLAG_TYPE = 0x00000004;
pub const DELTA_FLAG_EXPORTS: DELTA_FLAG_TYPE = 0x00000008;
pub const DELTA_FLAG_RESOURCES: DELTA_FLAG_TYPE = 0x00000010;
pub const DELTA_FLAG_RELOCS: DELTA_FLAG_TYPE = 0x00000020;
pub const DELTA_FLAG_I386_SMASHLOCK: DELTA_FLAG_TYPE = 0x00000040;
pub const DELTA_FLAG_I386_JMPS: DELTA_FLAG_TYPE = 0x00000080;
pub const DELTA_FLAG_I386_CALLS: DELTA_FLAG_TYPE = 0x00000100;
pub const DELTA_FLAG_AMD64_DISASM: DELTA_FLAG_TYPE = 0x00000200;
pub const DELTA_FLAG_AMD64_PDATA: DELTA_FLAG_TYPE = 0x00000400;
pub const DELTA_FLAG_IA64_DISASM: DELTA_FLAG_TYPE = 0x00000800;
pub const DELTA_FLAG_IA64_PDATA: DELTA_FLAG_TYPE = 0x00001000;
pub const DELTA_FLAG_UNBIND: DELTA_FLAG_TYPE = 0x00002000;
pub const DELTA_FLAG_CLI_DISASM: DELTA_FLAG_TYPE = 0x00004000;
pub const DELTA_FLAG_CLI_METADATA: DELTA_FLAG_TYPE = 0x00008000;
pub const DELTA_FLAG_HEADERS: DELTA_FLAG_TYPE = 0x00010000;
pub const DELTA_FLAG_IGNORE_FILE_SIZE_LIMIT: DELTA_FLAG_TYPE = 0x00020000;
pub const DELTA_FLAG_IGNORE_OPTIONS_SIZE_LIMIT: DELTA_FLAG_TYPE = 0x00040000;
pub const DELTA_FLAG_ARM_DISASM: DELTA_FLAG_TYPE = 0x00080000;
pub const DELTA_FLAG_ARM_PDATA: DELTA_FLAG_TYPE = 0x00100000;
pub const DELTA_FLAG_CLI4_METADATA: DELTA_FLAG_TYPE = 0x00200000;
pub const DELTA_FLAG_CLI4_DISASM: DELTA_FLAG_TYPE = 0x00400000;
pub const DELTA_FLAG_ARM64_DISASM: DELTA_FLAG_TYPE = 0x00800000;
pub const DELTA_FLAG_ARM64_PDATA: DELTA_FLAG_TYPE = 0x01000000;
pub const DELTA_DEFAULT_FLAGS_RAW: DELTA_FLAG_TYPE = DELTA_FLAG_NONE;
pub const DELTA_DEFAULT_FLAGS_I386: DELTA_FLAG_TYPE = DELTA_FLAG_MARK
                                                    | DELTA_FLAG_IMPORTS
                                                    | DELTA_FLAG_EXPORTS
                                                    | DELTA_FLAG_RESOURCES
                                                    | DELTA_FLAG_RELOCS
                                                    | DELTA_FLAG_I386_SMASHLOCK
                                                    | DELTA_FLAG_I386_JMPS
                                                    | DELTA_FLAG_I386_CALLS
                                                    | DELTA_FLAG_UNBIND
                                                    | DELTA_FLAG_CLI_DISASM
                                                    | DELTA_FLAG_CLI_METADATA;
pub const DELTA_DEFAULT_FLAGS_IA64: DELTA_FLAG_TYPE = DELTA_FLAG_MARK
                                                    | DELTA_FLAG_IMPORTS
                                                    | DELTA_FLAG_EXPORTS
                                                    | DELTA_FLAG_RESOURCES
                                                    | DELTA_FLAG_RELOCS
                                                    | DELTA_FLAG_IA64_DISASM
                                                    | DELTA_FLAG_IA64_PDATA
                                                    | DELTA_FLAG_UNBIND
                                                    | DELTA_FLAG_CLI_DISASM
                                                    | DELTA_FLAG_CLI_METADATA;
pub const DELTA_DEFAULT_FLAGS_AMD64: DELTA_FLAG_TYPE = DELTA_FLAG_MARK
                                                     | DELTA_FLAG_IMPORTS
                                                     | DELTA_FLAG_EXPORTS
                                                     | DELTA_FLAG_RESOURCES
                                                     | DELTA_FLAG_RELOCS
                                                     | DELTA_FLAG_AMD64_DISASM
                                                     | DELTA_FLAG_AMD64_PDATA
                                                     | DELTA_FLAG_UNBIND
                                                     | DELTA_FLAG_CLI_DISASM
                                                     | DELTA_FLAG_CLI_METADATA;
pub const DELTA_CLI4_FLAGS_I386: DELTA_FLAG_TYPE = DELTA_FLAG_MARK
                                                 | DELTA_FLAG_IMPORTS
                                                 | DELTA_FLAG_EXPORTS
                                                 | DELTA_FLAG_RESOURCES
                                                 | DELTA_FLAG_RELOCS
                                                 | DELTA_FLAG_I386_SMASHLOCK
                                                 | DELTA_FLAG_I386_JMPS
                                                 | DELTA_FLAG_I386_CALLS
                                                 | DELTA_FLAG_UNBIND
                                                 | DELTA_FLAG_CLI4_DISASM
                                                 | DELTA_FLAG_CLI4_METADATA;
pub const DELTA_CLI4_FLAGS_AMD64: DELTA_FLAG_TYPE = DELTA_FLAG_MARK
                                                  | DELTA_FLAG_IMPORTS
                                                  | DELTA_FLAG_EXPORTS
                                                  | DELTA_FLAG_RESOURCES
                                                  | DELTA_FLAG_RELOCS
                                                  | DELTA_FLAG_AMD64_DISASM
                                                  | DELTA_FLAG_AMD64_PDATA
                                                  | DELTA_FLAG_UNBIND
                                                  | DELTA_FLAG_CLI4_DISASM
                                                  | DELTA_FLAG_CLI4_METADATA;
pub const DELTA_CLI4_FLAGS_ARM: DELTA_FLAG_TYPE = DELTA_FLAG_MARK
                                                | DELTA_FLAG_IMPORTS
                                                | DELTA_FLAG_EXPORTS
                                                | DELTA_FLAG_RESOURCES
                                                | DELTA_FLAG_RELOCS
                                                | DELTA_FLAG_ARM_DISASM
                                                | DELTA_FLAG_ARM_PDATA
                                                | DELTA_FLAG_UNBIND
                                                | DELTA_FLAG_CLI4_DISASM
                                                | DELTA_FLAG_CLI4_METADATA;
pub const DELTA_CLI4_FLAGS_ARM64: DELTA_FLAG_TYPE = DELTA_FLAG_MARK
                                                  | DELTA_FLAG_IMPORTS
                                                  | DELTA_FLAG_EXPORTS
                                                  | DELTA_FLAG_RESOURCES
                                                  | DELTA_FLAG_RELOCS
                                                  | DELTA_FLAG_ARM64_DISASM
                                                  | DELTA_FLAG_ARM64_PDATA
                                                  | DELTA_FLAG_UNBIND
                                                  | DELTA_FLAG_CLI4_DISASM
                                                  | DELTA_FLAG_CLI4_METADATA;
pub const DELTA_MAX_HASH_SIZE: usize = 32;
STRUCT!{struct DELTA_HASH {
    HashSize: DWORD,
    HashValue: [UCHAR; DELTA_MAX_HASH_SIZE],
}}
pub type LPDELTA_HASH = *mut DELTA_HASH;
pub type LPCDELTA_HASH = *const DELTA_HASH;
STRUCT!{struct DELTA_HEADER_INFO {
    FileTypeSet: DELTA_FILE_TYPE,
    FileType: DELTA_FILE_TYPE,
    Flags: DELTA_FLAG_TYPE,
    TargetSize: SIZE_T,
    TargetFileTime: FILETIME,
    TargetHashAlgId: ALG_ID,
    TargetHash: DELTA_HASH,
}}
pub type LPDELTA_HEADER_INFO = *mut DELTA_HEADER_INFO;
pub type LPCDELTA_HEADER_INFO = *const DELTA_HEADER_INFO;
extern "system" {
    pub fn ApplyDeltaA(
        ApplyFlags: DELTA_FLAG_TYPE,
        lpSourceName: LPCSTR,
        lpDeltaName: LPCSTR,
        lpTargetName: LPCSTR,
    ) -> BOOL;
    pub fn ApplyDeltaB(
        ApplyFlags: DELTA_FLAG_TYPE,
        Source: DELTA_INPUT,
        Delta: DELTA_INPUT,
        lpTarget: LPDELTA_OUTPUT,
    ) -> BOOL;
    pub fn ApplyDeltaProvidedB(
        ApplyFlags: DELTA_FLAG_TYPE,
        Source: DELTA_INPUT,
        Delta: DELTA_INPUT,
        lpTarget: LPVOID,
        uTargetSize: SIZE_T,
    ) -> BOOL;
    pub fn ApplyDeltaW(
        ApplyFlags: DELTA_FLAG_TYPE,
        lpSourceName: LPCWSTR,
        lpDeltaName: LPCWSTR,
        lpTargetName: LPCWSTR,
    ) -> BOOL;
    pub fn CreateDeltaA(
        FileTypeSet: DELTA_FILE_TYPE,
        SetFlags: DELTA_FLAG_TYPE,
        ResetFlags: DELTA_FLAG_TYPE,
        lpSourceName: LPCSTR,
        lpTargetName: LPCSTR,
        lpSourceOptionsName: LPCSTR,
        lpTargetOptionsName: LPCSTR,
        GlobalOptions: DELTA_INPUT,
        lpTargetFileTime: *const FILETIME,
        HashAlgId: ALG_ID,
        lpDeltaName: LPCSTR,
    ) -> BOOL;
    pub fn CreateDeltaB(
        FileTypeSet: DELTA_FILE_TYPE,
        SetFlags: DELTA_FLAG_TYPE,
        ResetFlags: DELTA_FLAG_TYPE,
        Source: DELTA_INPUT,
        Target: DELTA_INPUT,
        SourceOptions: DELTA_INPUT,
        TargetOptions: DELTA_INPUT,
        GlobalOptions: DELTA_INPUT ,
        lpTargetFileTime: *const FILETIME,
        HashAlgId: ALG_ID,
        lpDelta: LPDELTA_OUTPUT,
    ) -> BOOL;
    pub fn CreateDeltaW(
        FileTypeSet: DELTA_FILE_TYPE,
        SetFlags: DELTA_FLAG_TYPE,
        ResetFlags: DELTA_FLAG_TYPE,
        lpSourceName: LPCWSTR,
        lpTargetName: LPCWSTR,
        lpSourceOptionsName: LPCWSTR,
        lpTargetOptionsName: LPCWSTR,
        GlobalOptions: DELTA_INPUT,
        lpTargetFileTime: *const FILETIME,
        HashAlgId: ALG_ID,
        lpDeltaName: LPCWSTR,
    ) -> BOOL;
    pub fn DeltaFree(
        lpMemory: LPVOID,
    ) -> BOOL;
    pub fn DeltaNormalizeProvidedB(
        FileTypeSet: DELTA_FILE_TYPE,
        NormalizeFlags: DELTA_FLAG_TYPE,
        NormalizeOptions: DELTA_INPUT,
        lpSource: LPVOID,
        uSourceSize: SIZE_T,
    ) -> BOOL;
    pub fn GetDeltaInfoA(
        Delta: DELTA_INPUT,
        lpHeaderInfo: LPDELTA_HEADER_INFO,
    ) -> BOOL;
    pub fn GetDeltaInfoB(
        Delta: DELTA_INPUT,
        lpHeaderInfo: LPDELTA_HEADER_INFO,
    ) -> BOOL;
    pub fn GetDeltaInfoW(
        lpDeltaName: LPCWSTR,
        lpHeaderInfo: LPDELTA_HEADER_INFO,
    ) -> BOOL;
    pub fn GetDeltaSignatureA(
        FileTypeSet: DELTA_FILE_TYPE,
        HashAlgId: ALG_ID,
        lpSourceName: LPCSTR,
        lpHash: LPDELTA_HASH,
    ) -> BOOL;
    pub fn GetDeltaSignatureB(
        FileTypeSet: DELTA_FILE_TYPE,
        HashAlgId: ALG_ID,
        Source: DELTA_INPUT,
        lpHash: LPDELTA_HASH,
    ) -> BOOL;
    pub fn GetDeltaSignatureW(
        FileTypeSet: DELTA_FILE_TYPE,
        HashAlgId: ALG_ID,
        lpSourceName: LPCWSTR,
        lpHash: LPDELTA_HASH,
    ) -> BOOL;
}
