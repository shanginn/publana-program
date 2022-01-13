#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use anchor_lang::prelude::*;
use borsh::{BorshSerialize, BorshDeserialize};
use anchor_lang::solana_program::{
    system_instruction, system_program, program::invoke_signed, native_token::lamports_to_sol,
};
use std::mem::size_of;
/// The static program ID
pub static ID: anchor_lang::solana_program::pubkey::Pubkey =
    anchor_lang::solana_program::pubkey::Pubkey::new_from_array([
        228u8, 62u8, 171u8, 106u8, 17u8, 247u8, 34u8, 155u8, 151u8, 221u8, 86u8, 31u8, 167u8,
        164u8, 22u8, 68u8, 234u8, 186u8, 201u8, 236u8, 67u8, 17u8, 65u8, 149u8, 239u8, 190u8,
        159u8, 158u8, 114u8, 143u8, 75u8, 95u8,
    ]);
/// Confirms that a given pubkey is equivalent to the program ID
pub fn check_id(id: &anchor_lang::solana_program::pubkey::Pubkey) -> bool {
    id == &ID
}
/// Returns the program ID
pub fn id() -> anchor_lang::solana_program::pubkey::Pubkey {
    ID
}
/// Anchor generated Result to be used as the return type for the
/// program.
pub type Result<T> = std::result::Result<T, Error>;
/// Anchor generated error allowing one to easily return a
/// `ProgramError` or a custom, user defined error code by utilizing
/// its `From` implementation.
#[doc(hidden)]
pub enum Error {
    #[error(transparent)]
    ProgramError(#[from] anchor_lang::solana_program::program_error::ProgramError),
    #[error(transparent)]
    ErrorCode(#[from] ErrorCode),
}
#[allow(unused_qualifications)]
impl std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        use thiserror::private::AsDynError;
        #[allow(deprecated)]
        match self {
            Error::ProgramError { 0: transparent } => {
                std::error::Error::source(transparent.as_dyn_error())
            }
            Error::ErrorCode { 0: transparent } => {
                std::error::Error::source(transparent.as_dyn_error())
            }
        }
    }
}
#[allow(unused_qualifications)]
impl std::fmt::Display for Error {
    fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
        match self {
            Error::ProgramError(_0) => std::fmt::Display::fmt(_0, __formatter),
            Error::ErrorCode(_0) => std::fmt::Display::fmt(_0, __formatter),
        }
    }
}
#[allow(unused_qualifications)]
impl std::convert::From<anchor_lang::solana_program::program_error::ProgramError> for Error {
    #[allow(deprecated)]
    fn from(source: anchor_lang::solana_program::program_error::ProgramError) -> Self {
        Error::ProgramError { 0: source }
    }
}
#[allow(unused_qualifications)]
impl std::convert::From<ErrorCode> for Error {
    #[allow(deprecated)]
    fn from(source: ErrorCode) -> Self {
        Error::ErrorCode { 0: source }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Error {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&Error::ProgramError(ref __self_0),) => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_tuple(f, "ProgramError");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&Error::ErrorCode(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "ErrorCode");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
#[repr(u32)]
pub enum ErrorCode {
    TitleTooLong,
    TextLimitExceeded,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for ErrorCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&ErrorCode::TitleTooLong,) => ::core::fmt::Formatter::write_str(f, "TitleTooLong"),
            (&ErrorCode::TextLimitExceeded,) => {
                ::core::fmt::Formatter::write_str(f, "TextLimitExceeded")
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for ErrorCode {
    #[inline]
    fn clone(&self) -> ErrorCode {
        {
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for ErrorCode {}
impl std::fmt::Display for ErrorCode {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            ErrorCode::TitleTooLong => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["The provided title should be 280 characters long maximum."],
                &match () {
                    _args => [],
                },
            )),
            ErrorCode::TextLimitExceeded => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Can not update Ad. Text limit will be exceeded."],
                &match () {
                    _args => [],
                },
            )),
        }
    }
}
impl std::error::Error for ErrorCode {}
impl std::convert::From<Error> for anchor_lang::solana_program::program_error::ProgramError {
    fn from(e: Error) -> anchor_lang::solana_program::program_error::ProgramError {
        match e {
            Error::ProgramError(e) => e,
            Error::ErrorCode(c) => {
                anchor_lang::solana_program::program_error::ProgramError::Custom(
                    c as u32 + anchor_lang::__private::ERROR_CODE_OFFSET,
                )
            }
        }
    }
}
impl std::convert::From<ErrorCode> for anchor_lang::solana_program::program_error::ProgramError {
    fn from(e: ErrorCode) -> anchor_lang::solana_program::program_error::ProgramError {
        let err: Error = e.into();
        err.into()
    }
}
const KOLYAN_ACCOUNT: Pubkey = Pubkey::new_from_array([
    10u8, 115u8, 233u8, 51u8, 5u8, 207u8, 142u8, 21u8, 147u8, 47u8, 224u8, 44u8, 100u8, 226u8,
    179u8, 253u8, 113u8, 232u8, 83u8, 245u8, 34u8, 17u8, 223u8, 237u8, 29u8, 185u8, 74u8, 230u8,
    0u8, 72u8, 12u8, 91u8,
]);
pub static VIKTRCH_ACCOUNT: Pubkey = Pubkey::new_from_array([
    148u8, 222u8, 7u8, 85u8, 28u8, 77u8, 51u8, 150u8, 49u8, 109u8, 4u8, 45u8, 57u8, 197u8, 87u8,
    61u8, 177u8, 218u8, 67u8, 51u8, 111u8, 121u8, 105u8, 152u8, 192u8, 188u8, 199u8, 160u8, 36u8,
    167u8, 22u8, 120u8,
]);
const MAX_TITLE_LENGTH: usize = 280;
const BASE_TAX_LAMPORTS: u64 = 100000;
const TAX_PER_RANK_LAMPORTS: u64 = 10000;
use solana_ads::*;
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let (program_id, accounts, instruction_data) =
        unsafe { ::solana_program::entrypoint::deserialize(input) };
    match entry(&program_id, &accounts, &instruction_data) {
        Ok(()) => ::solana_program::entrypoint::SUCCESS,
        Err(error) => error.into(),
    }
}
/// The Anchor codegen exposes a programming model where a user defines
/// a set of methods inside of a `#[program]` module in a way similar
/// to writing RPC request handlers. The macro then generates a bunch of
/// code wrapping these user defined methods into something that can be
/// executed on Solana.
///
/// These methods fall into one of three categories, each of which
/// can be considered a different "namespace" of the program.
///
/// 1) Global methods - regular methods inside of the `#[program]`.
/// 2) State methods - associated methods inside a `#[state]` struct.
/// 3) Interface methods - methods inside a strait struct's
///    implementation of an `#[interface]` trait.
///
/// Care must be taken by the codegen to prevent collisions between
/// methods in these different namespaces. For this reason, Anchor uses
/// a variant of sighash to perform method dispatch, rather than
/// something like a simple enum variant discriminator.
///
/// The execution flow of the generated code can be roughly outlined:
///
/// * Start program via the entrypoint.
/// * Strip method identifier off the first 8 bytes of the instruction
///   data and invoke the identified method. The method identifier
///   is a variant of sighash. See docs.rs for `anchor_lang` for details.
/// * If the method identifier is an IDL identifier, execute the IDL
///   instructions, which are a special set of hardcoded instructions
///   baked into every Anchor program. Then exit.
/// * Otherwise, the method identifier is for a user defined
///   instruction, i.e., one of the methods in the user defined
///   `#[program]` module. Perform method dispatch, i.e., execute the
///   big match statement mapping method identifier to method handler
///   wrapper.
/// * Run the method handler wrapper. This wraps the code the user
///   actually wrote, deserializing the accounts, constructing the
///   context, invoking the user's code, and finally running the exit
///   routine, which typically persists account changes.
///
/// The `entry` function here, defines the standard entry to a Solana
/// program, where execution begins.
#[cfg(not(feature = "no-entrypoint"))]
pub fn entry(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    if data.len() < 8 {
        return Err(anchor_lang::__private::ErrorCode::InstructionMissing.into());
    }
    dispatch(program_id, accounts, data).map_err(|e| {
        ::solana_program::log::sol_log(&e.to_string());
        e
    })
}
pub mod program {
    use super::*;
    /// Type representing the program.
    pub struct SolanaAds;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for SolanaAds {
        #[inline]
        fn clone(&self) -> SolanaAds {
            match *self {
                SolanaAds => SolanaAds,
            }
        }
    }
    impl anchor_lang::AccountDeserialize for SolanaAds {
        fn try_deserialize(
            buf: &mut &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            SolanaAds::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(
            _buf: &mut &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            Ok(SolanaAds)
        }
    }
    impl anchor_lang::Id for SolanaAds {
        fn id() -> Pubkey {
            ID
        }
    }
}
/// Performs method dispatch.
///
/// Each method in an anchor program is uniquely defined by a namespace
/// and a rust identifier (i.e., the name given to the method). These
/// two pieces can be combined to creater a method identifier,
/// specifically, Anchor uses
///
/// Sha256("<namespace>::<rust-identifier>")[..8],
///
/// where the namespace can be one of three types. 1) "global" for a
/// regular instruction, 2) "state" for a state struct instruction
/// handler and 3) a trait namespace (used in combination with the
/// `#[interface]` attribute), which is defined by the trait name, e..
/// `MyTrait`.
///
/// With this 8 byte identifier, Anchor performs method dispatch,
/// matching the given 8 byte identifier to the associated method
/// handler, which leads to user defined code being eventually invoked.
fn dispatch(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let mut ix_data: &[u8] = data;
    let sighash: [u8; 8] = {
        let mut sighash: [u8; 8] = [0; 8];
        sighash.copy_from_slice(&ix_data[..8]);
        ix_data = &ix_data[8..];
        sighash
    };
    if true {
        if sighash == anchor_lang::idl::IDL_IX_TAG.to_le_bytes() {
            return __private::__idl::__idl_dispatch(program_id, accounts, &ix_data);
        }
    }
    match sighash {
        [103, 133, 51, 53, 3, 145, 44, 142] => {
            __private::__global::create_ad(program_id, accounts, ix_data)
        }
        [28, 63, 46, 111, 92, 161, 247, 193] => {
            __private::__global::update_ad(program_id, accounts, ix_data)
        }
        [212, 39, 81, 12, 19, 96, 71, 159] => {
            __private::__global::append_ad_content(program_id, accounts, ix_data)
        }
        [139, 232, 52, 103, 3, 239, 120, 42] => {
            __private::__global::delete_ad(program_id, accounts, ix_data)
        }
        _ => Err(anchor_lang::__private::ErrorCode::InstructionFallbackNotFound.into()),
    }
}
/// Create a private module to not clutter the program's namespace.
/// Defines an entrypoint for each individual instruction handler
/// wrapper.
mod __private {
    use super::*;
    /// __idl mod defines handlers for injected Anchor IDL instructions.
    pub mod __idl {
        use super::*;
        #[inline(never)]
        #[cfg(not(feature = "no-idl"))]
        pub fn __idl_dispatch(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            idl_ix_data: &[u8],
        ) -> ProgramResult {
            let mut accounts = accounts;
            let mut data: &[u8] = idl_ix_data;
            let ix = anchor_lang::idl::IdlInstruction::deserialize(&mut data)
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            match ix {
                anchor_lang::idl::IdlInstruction::Create { data_len } => {
                    let mut accounts = anchor_lang::idl::IdlCreateAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_create_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::CreateBuffer => {
                    let mut accounts = anchor_lang::idl::IdlCreateBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_create_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Write { data } => {
                    let mut accounts = anchor_lang::idl::IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_write(program_id, &mut accounts, data)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetAuthority { new_authority } => {
                    let mut accounts = anchor_lang::idl::IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_set_authority(program_id, &mut accounts, new_authority)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetBuffer => {
                    let mut accounts = anchor_lang::idl::IdlSetBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_set_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
            }
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_account(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlCreateAccounts,
            data_len: u64,
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: IdlCreateAccount");
            if program_id != accounts.program.key {
                return Err(anchor_lang::__private::ErrorCode::IdlInstructionInvalidProgram.into());
            }
            let from = accounts.from.key;
            let (base, nonce) = Pubkey::find_program_address(&[], program_id);
            let seed = anchor_lang::idl::IdlAccount::seed();
            let owner = accounts.program.key;
            let to = Pubkey::create_with_seed(&base, seed, owner).unwrap();
            let space = 8 + 32 + 4 + data_len as usize;
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(space);
            let seeds = &[&[nonce][..]];
            let ix = anchor_lang::solana_program::system_instruction::create_account_with_seed(
                from,
                &to,
                &base,
                seed,
                lamports,
                space as u64,
                owner,
            );
            anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &[
                    accounts.from.clone(),
                    accounts.to.clone(),
                    accounts.base.clone(),
                    accounts.system_program.clone(),
                ],
                &[seeds],
            )?;
            let mut idl_account = {
                let mut account_data = accounts.to.try_borrow_data()?;
                let mut account_data_slice: &[u8] = &account_data;
                anchor_lang::idl::IdlAccount::try_deserialize_unchecked(&mut account_data_slice)?
            };
            idl_account.authority = *accounts.from.key;
            let mut data = accounts.to.try_borrow_mut_data()?;
            let dst: &mut [u8] = &mut data;
            let mut cursor = std::io::Cursor::new(dst);
            idl_account.try_serialize(&mut cursor)?;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_buffer(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlCreateBuffer,
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: IdlCreateBuffer");
            let mut buffer = &mut accounts.buffer;
            buffer.authority = *accounts.authority.key;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_write(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlAccounts,
            idl_data: Vec<u8>,
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: IdlWrite");
            let mut idl = &mut accounts.idl;
            idl.data.extend(idl_data);
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_authority(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlAccounts,
            new_authority: Pubkey,
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: IdlSetAuthority");
            accounts.idl.authority = new_authority;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_buffer(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlSetBuffer,
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: IdlSetBuffer");
            accounts.idl.data = accounts.buffer.data.clone();
            Ok(())
        }
    }
    /// __state mod defines wrapped handlers for state instructions.
    pub mod __state {
        use super::*;
    }
    /// __interface mod defines wrapped handlers for `#[interface]` trait
    /// implementations.
    pub mod __interface {
        use super::*;
    }
    /// __global mod defines wrapped handlers for global instructions.
    pub mod __global {
        use super::*;
        #[inline(never)]
        pub fn create_ad(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: CreateAd");
            let ix = instruction::CreateAd::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::CreateAd {
                title,
                content,
                text_limit,
                rank,
            } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                CreateAd::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            solana_ads::create_ad(
                Context::new(program_id, &mut accounts, remaining_accounts),
                title,
                content,
                text_limit,
                rank,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn update_ad(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: UpdateAd");
            let ix = instruction::UpdateAd::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::UpdateAd { title, content } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                UpdateAd::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            solana_ads::update_ad(
                Context::new(program_id, &mut accounts, remaining_accounts),
                title,
                content,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn append_ad_content(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: AppendAdContent");
            let ix = instruction::AppendAdContent::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::AppendAdContent { content } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                AppendAdContent::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            solana_ads::append_ad_content(
                Context::new(program_id, &mut accounts, remaining_accounts),
                content,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn delete_ad(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: DeleteAd");
            let ix = instruction::DeleteAd::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::DeleteAd = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                DeleteAd::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            solana_ads::delete_ad(Context::new(program_id, &mut accounts, remaining_accounts))?;
            accounts.exit(program_id)
        }
    }
}
pub mod solana_ads {
    use super::*;
    pub fn create_ad(
        ctx: Context<CreateAd>,
        title: String,
        content: String,
        text_limit: u32,
        rank: u64,
    ) -> ProgramResult {
        let ad = &mut ctx.accounts.ad;
        let authority = &ctx.accounts.authority;
        let clock: Clock = Clock::get().unwrap();
        ::solana_program::log::sol_log("Begin creating ad");
        if title.chars().count() > MAX_TITLE_LENGTH {
            return Err(ErrorCode::TitleTooLong.into());
        }
        ad.title = title;
        ad.content = content;
        ad.authority = authority.key();
        ad.timestamp = clock.unix_timestamp;
        ad.text_limit = text_limit;
        ad.rank = rank;
        let kolyan_account = &ctx.accounts.kolyan_account;
        let viktrch_account = &ctx.accounts.viktrch_account;
        let tax = BASE_TAX_LAMPORTS + TAX_PER_RANK_LAMPORTS * rank;
        ::solana_program::log::sol_log(&{
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["Tax: ", " SOL"],
                &match (&lamports_to_sol(tax),) {
                    _args => [::core::fmt::ArgumentV1::new(
                        _args.0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ));
            res
        });
        invoke_signed(
            &system_instruction::transfer(&authority.key(), &kolyan_account.key(), tax / 2),
            &[
                authority.to_account_info(),
                kolyan_account.to_account_info(),
            ],
            &[],
        )
        .unwrap();
        ::solana_program::log::sol_log("Half sent to Kolyan");
        invoke_signed(
            &system_instruction::transfer(&authority.key(), &viktrch_account.key(), tax / 2),
            &[
                authority.to_account_info(),
                viktrch_account.to_account_info(),
            ],
            &[],
        )
        .unwrap();
        ::solana_program::log::sol_log("Half sent to Victrch");
        Ok(())
    }
    pub fn update_ad(ctx: Context<UpdateAd>, title: String, content: String) -> ProgramResult {
        let ad = &mut ctx.accounts.ad;
        if title.chars().count() as u32 + content.chars().count() as u32 > ad.text_limit {
            return Err(ErrorCode::TextLimitExceeded.into());
        }
        ad.title = title;
        ad.content = content;
        Ok(())
    }
    pub fn append_ad_content(ctx: Context<AppendAdContent>, content: String) -> ProgramResult {
        let ad = &mut ctx.accounts.ad;
        if ad.title.chars().count() as u32
            + ad.content.chars().count() as u32
            + content.chars().count() as u32
            > ad.text_limit
        {
            return Err(ErrorCode::TextLimitExceeded.into());
        }
        ad.content += &content;
        Ok(())
    }
    pub fn delete_ad(ctx: Context<DeleteAd>) -> ProgramResult {
        let _ad = &mut ctx.accounts.ad;
        Ok(())
    }
}
/// An Anchor generated module containing the program's set of
/// instructions, where each method handler in the `#[program]` mod is
/// associated with a struct defining the input arguments to the
/// method. These should be used directly, when one wants to serialize
/// Anchor instruction data, for example, when speciying
/// instructions on a client.
pub mod instruction {
    use super::*;
    /// Instruction struct definitions for `#[state]` methods.
    pub mod state {
        use super::*;
    }
    /// Instruction.
    pub struct CreateAd {
        pub title: String,
        pub content: String,
        pub text_limit: u32,
        pub rank: u64,
    }
    impl borsh::ser::BorshSerialize for CreateAd
    where
        String: borsh::ser::BorshSerialize,
        String: borsh::ser::BorshSerialize,
        u32: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.title, writer)?;
            borsh::BorshSerialize::serialize(&self.content, writer)?;
            borsh::BorshSerialize::serialize(&self.text_limit, writer)?;
            borsh::BorshSerialize::serialize(&self.rank, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for CreateAd
    where
        String: borsh::BorshDeserialize,
        String: borsh::BorshDeserialize,
        u32: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                title: borsh::BorshDeserialize::deserialize(buf)?,
                content: borsh::BorshDeserialize::deserialize(buf)?,
                text_limit: borsh::BorshDeserialize::deserialize(buf)?,
                rank: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for CreateAd {
        fn data(&self) -> Vec<u8> {
            let mut d = [103, 133, 51, 53, 3, 145, 44, 142].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct UpdateAd {
        pub title: String,
        pub content: String,
    }
    impl borsh::ser::BorshSerialize for UpdateAd
    where
        String: borsh::ser::BorshSerialize,
        String: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.title, writer)?;
            borsh::BorshSerialize::serialize(&self.content, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for UpdateAd
    where
        String: borsh::BorshDeserialize,
        String: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                title: borsh::BorshDeserialize::deserialize(buf)?,
                content: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for UpdateAd {
        fn data(&self) -> Vec<u8> {
            let mut d = [28, 63, 46, 111, 92, 161, 247, 193].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct AppendAdContent {
        pub content: String,
    }
    impl borsh::ser::BorshSerialize for AppendAdContent
    where
        String: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.content, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for AppendAdContent
    where
        String: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                content: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for AppendAdContent {
        fn data(&self) -> Vec<u8> {
            let mut d = [212, 39, 81, 12, 19, 96, 71, 159].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct DeleteAd;
    impl borsh::ser::BorshSerialize for DeleteAd {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for DeleteAd {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for DeleteAd {
        fn data(&self) -> Vec<u8> {
            let mut d = [139, 232, 52, 103, 3, 239, 120, 42].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
}
/// An Anchor generated module, providing a set of structs
/// mirroring the structs deriving `Accounts`, where each field is
/// a `Pubkey`. This is useful for specifying accounts for a client.
pub mod accounts {
    pub use crate::__client_accounts_append_ad_content::*;
    pub use crate::__client_accounts_delete_ad::*;
    pub use crate::__client_accounts_create_ad::*;
    pub use crate::__client_accounts_update_ad::*;
}
pub struct CreateAd<'info> {
    # [account (init , payer = authority , space = Ad :: size (ix_data))]
    pub ad: Account<'info, Ad>,
    #[account(mut)]
    pub authority: Signer<'info>,
    # [account (address = system_program :: ID)]
    pub system_program: Program<'info, System>,
    # [account (mut , address = KOLYAN_ACCOUNT)]
    pub kolyan_account: UncheckedAccount<'info>,
    # [account (mut , address = VIKTRCH_ACCOUNT)]
    pub viktrch_account: UncheckedAccount<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for CreateAd<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let ad = &accounts[0];
        *accounts = &accounts[1..];
        let authority: Signer = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let system_program: anchor_lang::Program<System> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let kolyan_account: UncheckedAccount =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let viktrch_account: UncheckedAccount =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let __anchor_rent = Rent::get()?;
        let ad = {
            let actual_field = ad.to_account_info();
            let actual_owner = actual_field.owner;
            let space = Ad::size(ix_data);
            if !false || actual_owner == &anchor_lang::solana_program::system_program::ID {
                let payer = authority.to_account_info();
                let __current_lamports = ad.to_account_info().lamports();
                if __current_lamports == 0 {
                    let lamports = __anchor_rent.minimum_balance(space);
                    anchor_lang::solana_program::program::invoke_signed(
                        &anchor_lang::solana_program::system_instruction::create_account(
                            payer.to_account_info().key,
                            ad.to_account_info().key,
                            lamports,
                            space as u64,
                            program_id,
                        ),
                        &[
                            payer.to_account_info(),
                            ad.to_account_info(),
                            system_program.to_account_info(),
                        ],
                        &[],
                    )?;
                } else {
                    let required_lamports = __anchor_rent
                        .minimum_balance(space)
                        .max(1)
                        .saturating_sub(__current_lamports);
                    if required_lamports > 0 {
                        anchor_lang::solana_program::program::invoke(
                            &anchor_lang::solana_program::system_instruction::transfer(
                                payer.to_account_info().key,
                                ad.to_account_info().key,
                                required_lamports,
                            ),
                            &[
                                payer.to_account_info(),
                                ad.to_account_info(),
                                system_program.to_account_info(),
                            ],
                        )?;
                    }
                    anchor_lang::solana_program::program::invoke_signed(
                        &anchor_lang::solana_program::system_instruction::allocate(
                            ad.to_account_info().key,
                            space as u64,
                        ),
                        &[ad.to_account_info(), system_program.to_account_info()],
                        &[],
                    )?;
                    anchor_lang::solana_program::program::invoke_signed(
                        &anchor_lang::solana_program::system_instruction::assign(
                            ad.to_account_info().key,
                            program_id,
                        ),
                        &[ad.to_account_info(), system_program.to_account_info()],
                        &[],
                    )?;
                }
            }
            let pa: anchor_lang::Account<Ad> = anchor_lang::Account::try_from_unchecked(&ad)?;
            if !(!false || actual_owner == &anchor_lang::solana_program::system_program::ID) {
                if space != actual_field.data_len() {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSpace.into());
                }
                if actual_owner != program_id {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintOwner.into());
                }
            }
            pa
        };
        if !ad.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !ad.to_account_info().is_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
        }
        if !__anchor_rent.is_exempt(
            ad.to_account_info().lamports(),
            ad.to_account_info().try_data_len()?,
        ) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
        }
        if !authority.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if system_program.to_account_info().key != &system_program::ID {
            return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
        }
        if !kolyan_account.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if kolyan_account.to_account_info().key != &KOLYAN_ACCOUNT {
            return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
        }
        if !viktrch_account.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if viktrch_account.to_account_info().key != &VIKTRCH_ACCOUNT {
            return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
        }
        Ok(CreateAd {
            ad,
            authority,
            system_program,
            kolyan_account,
            viktrch_account,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for CreateAd<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.ad.to_account_infos());
        account_infos.extend(self.authority.to_account_infos());
        account_infos.extend(self.system_program.to_account_infos());
        account_infos.extend(self.kolyan_account.to_account_infos());
        account_infos.extend(self.viktrch_account.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for CreateAd<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.ad.to_account_metas(Some(true)));
        account_metas.extend(self.authority.to_account_metas(None));
        account_metas.extend(self.system_program.to_account_metas(None));
        account_metas.extend(self.kolyan_account.to_account_metas(None));
        account_metas.extend(self.viktrch_account.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for CreateAd<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.ad, program_id)?;
        anchor_lang::AccountsExit::exit(&self.authority, program_id)?;
        anchor_lang::AccountsExit::exit(&self.kolyan_account, program_id)?;
        anchor_lang::AccountsExit::exit(&self.viktrch_account, program_id)?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_create_ad {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct CreateAd {
        pub ad: anchor_lang::solana_program::pubkey::Pubkey,
        pub authority: anchor_lang::solana_program::pubkey::Pubkey,
        pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub kolyan_account: anchor_lang::solana_program::pubkey::Pubkey,
        pub viktrch_account: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for CreateAd
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.ad, writer)?;
            borsh::BorshSerialize::serialize(&self.authority, writer)?;
            borsh::BorshSerialize::serialize(&self.system_program, writer)?;
            borsh::BorshSerialize::serialize(&self.kolyan_account, writer)?;
            borsh::BorshSerialize::serialize(&self.viktrch_account, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for CreateAd {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.ad, true,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.authority,
                true,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.system_program,
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.kolyan_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.viktrch_account,
                false,
            ));
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// `cpi::accounts` module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_create_ad {
    use super::*;
    pub struct CreateAd<'info> {
        pub ad: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub kolyan_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub viktrch_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for CreateAd<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.ad),
                true,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.authority),
                true,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.system_program),
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.kolyan_account),
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.viktrch_account),
                false,
            ));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for CreateAd<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.ad));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.authority));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.system_program,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.kolyan_account,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.viktrch_account,
            ));
            account_infos
        }
    }
}
pub struct UpdateAd<'info> {
    # [account (mut , has_one = authority)]
    pub ad: Account<'info, Ad>,
    pub authority: Signer<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for UpdateAd<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let ad: anchor_lang::Account<Ad> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let authority: Signer = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if !ad.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if &ad.authority != authority.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        Ok(UpdateAd { ad, authority })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for UpdateAd<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.ad.to_account_infos());
        account_infos.extend(self.authority.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for UpdateAd<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.ad.to_account_metas(None));
        account_metas.extend(self.authority.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for UpdateAd<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.ad, program_id)?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_update_ad {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct UpdateAd {
        pub ad: anchor_lang::solana_program::pubkey::Pubkey,
        pub authority: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for UpdateAd
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.ad, writer)?;
            borsh::BorshSerialize::serialize(&self.authority, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for UpdateAd {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.ad, false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.authority,
                    true,
                ),
            );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// `cpi::accounts` module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_update_ad {
    use super::*;
    pub struct UpdateAd<'info> {
        pub ad: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for UpdateAd<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.ad),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.authority),
                    true,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for UpdateAd<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.ad));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.authority));
            account_infos
        }
    }
}
pub struct DeleteAd<'info> {
    # [account (mut , has_one = authority , close = authority ,)]
    pub ad: Account<'info, Ad>,
    #[account(mut)]
    pub authority: Signer<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for DeleteAd<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let ad: anchor_lang::Account<Ad> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let authority: Signer = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if !ad.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if &ad.authority != authority.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if ad.to_account_info().key == authority.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintClose.into());
        }
        if !authority.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        Ok(DeleteAd { ad, authority })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for DeleteAd<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.ad.to_account_infos());
        account_infos.extend(self.authority.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for DeleteAd<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.ad.to_account_metas(None));
        account_metas.extend(self.authority.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for DeleteAd<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsClose::close(&self.ad, self.authority.to_account_info())?;
        anchor_lang::AccountsExit::exit(&self.authority, program_id)?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_delete_ad {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct DeleteAd {
        pub ad: anchor_lang::solana_program::pubkey::Pubkey,
        pub authority: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for DeleteAd
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.ad, writer)?;
            borsh::BorshSerialize::serialize(&self.authority, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for DeleteAd {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.ad, false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.authority,
                true,
            ));
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// `cpi::accounts` module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_delete_ad {
    use super::*;
    pub struct DeleteAd<'info> {
        pub ad: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for DeleteAd<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.ad),
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.authority),
                true,
            ));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for DeleteAd<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.ad));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.authority));
            account_infos
        }
    }
}
pub struct AppendAdContent<'info> {
    # [account (mut , has_one = authority)]
    pub ad: Account<'info, Ad>,
    pub authority: Signer<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for AppendAdContent<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let ad: anchor_lang::Account<Ad> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let authority: Signer = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if !ad.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if &ad.authority != authority.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        Ok(AppendAdContent { ad, authority })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for AppendAdContent<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.ad.to_account_infos());
        account_infos.extend(self.authority.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for AppendAdContent<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.ad.to_account_metas(None));
        account_metas.extend(self.authority.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for AppendAdContent<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.ad, program_id)?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_append_ad_content {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct AppendAdContent {
        pub ad: anchor_lang::solana_program::pubkey::Pubkey,
        pub authority: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for AppendAdContent
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.ad, writer)?;
            borsh::BorshSerialize::serialize(&self.authority, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for AppendAdContent {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.ad, false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.authority,
                    true,
                ),
            );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// `cpi::accounts` module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_append_ad_content {
    use super::*;
    pub struct AppendAdContent<'info> {
        pub ad: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for AppendAdContent<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.ad),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.authority),
                    true,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for AppendAdContent<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.ad));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.authority));
            account_infos
        }
    }
}
pub struct Ad {
    pub title: String,
    pub content: String,
    pub timestamp: i64,
    pub authority: Pubkey,
    pub text_limit: u32,
    pub rank: u64,
}
impl borsh::ser::BorshSerialize for Ad
where
    String: borsh::ser::BorshSerialize,
    String: borsh::ser::BorshSerialize,
    i64: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    u32: borsh::ser::BorshSerialize,
    u64: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.title, writer)?;
        borsh::BorshSerialize::serialize(&self.content, writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, writer)?;
        borsh::BorshSerialize::serialize(&self.authority, writer)?;
        borsh::BorshSerialize::serialize(&self.text_limit, writer)?;
        borsh::BorshSerialize::serialize(&self.rank, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for Ad
where
    String: borsh::BorshDeserialize,
    String: borsh::BorshDeserialize,
    i64: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    u32: borsh::BorshDeserialize,
    u64: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            title: borsh::BorshDeserialize::deserialize(buf)?,
            content: borsh::BorshDeserialize::deserialize(buf)?,
            timestamp: borsh::BorshDeserialize::deserialize(buf)?,
            authority: borsh::BorshDeserialize::deserialize(buf)?,
            text_limit: borsh::BorshDeserialize::deserialize(buf)?,
            rank: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Ad {
    #[inline]
    fn clone(&self) -> Ad {
        match *self {
            Ad {
                title: ref __self_0_0,
                content: ref __self_0_1,
                timestamp: ref __self_0_2,
                authority: ref __self_0_3,
                text_limit: ref __self_0_4,
                rank: ref __self_0_5,
            } => Ad {
                title: ::core::clone::Clone::clone(&(*__self_0_0)),
                content: ::core::clone::Clone::clone(&(*__self_0_1)),
                timestamp: ::core::clone::Clone::clone(&(*__self_0_2)),
                authority: ::core::clone::Clone::clone(&(*__self_0_3)),
                text_limit: ::core::clone::Clone::clone(&(*__self_0_4)),
                rank: ::core::clone::Clone::clone(&(*__self_0_5)),
            },
        }
    }
}
#[automatically_derived]
impl anchor_lang::AccountSerialize for Ad {
    fn try_serialize<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> std::result::Result<(), ProgramError> {
        writer
            .write_all(&[81, 91, 73, 106, 215, 137, 214, 47])
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        AnchorSerialize::serialize(self, writer)
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        Ok(())
    }
}
#[automatically_derived]
impl anchor_lang::AccountDeserialize for Ad {
    fn try_deserialize(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
        if buf.len() < [81, 91, 73, 106, 215, 137, 214, 47].len() {
            return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorNotFound.into());
        }
        let given_disc = &buf[..8];
        if &[81, 91, 73, 106, 215, 137, 214, 47] != given_disc {
            return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorMismatch.into());
        }
        Self::try_deserialize_unchecked(buf)
    }
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
        let mut data: &[u8] = &buf[8..];
        AnchorDeserialize::deserialize(&mut data)
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotDeserialize.into())
    }
}
#[automatically_derived]
impl anchor_lang::Discriminator for Ad {
    fn discriminator() -> [u8; 8] {
        [81, 91, 73, 106, 215, 137, 214, 47]
    }
}
#[automatically_derived]
impl anchor_lang::Owner for Ad {
    fn owner() -> Pubkey {
        crate::ID
    }
}
const DISCRIMINATOR_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
impl Ad {
    pub fn size(ix_data: &[u8]) -> usize {
        ::solana_program::log::sol_log("Calculating account size");
        struct CreateAdArgs {
            pub title: String,
            pub content: String,
            pub text_len: u32,
            pub rank: u64,
        }
        impl borsh::de::BorshDeserialize for CreateAdArgs
        where
            String: borsh::BorshDeserialize,
            String: borsh::BorshDeserialize,
            u32: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    title: borsh::BorshDeserialize::deserialize(buf)?,
                    content: borsh::BorshDeserialize::deserialize(buf)?,
                    text_len: borsh::BorshDeserialize::deserialize(buf)?,
                    rank: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        impl borsh::ser::BorshSerialize for CreateAdArgs
        where
            String: borsh::ser::BorshSerialize,
            String: borsh::ser::BorshSerialize,
            u32: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.title, writer)?;
                borsh::BorshSerialize::serialize(&self.content, writer)?;
                borsh::BorshSerialize::serialize(&self.text_len, writer)?;
                borsh::BorshSerialize::serialize(&self.rank, writer)?;
                Ok(())
            }
        }
        let create_ad_args: CreateAdArgs = CreateAdArgs::try_from_slice(&ix_data[..]).unwrap();
        let size = DISCRIMINATOR_LENGTH
            + STRING_LENGTH_PREFIX
            + STRING_LENGTH_PREFIX
            + (create_ad_args.text_len as usize) * 4
            + size_of::<i64>()
            + size_of::<u64>()
            + size_of::<Pubkey>()
            + size_of::<u32>();
        ::solana_program::log::sol_log(&{
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["Account size: "],
                &match (&size,) {
                    _args => [::core::fmt::ArgumentV1::new(
                        _args.0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ));
            res
        });
        size
    }
}
