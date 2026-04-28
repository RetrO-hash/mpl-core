use num_traits::ToPrimitive;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, rent::Rent,
    system_instruction, sysvar::Sysvar,
};

use crate::{error::MplCoreError, state::Key};

pub(crate) fn close_program_account<'a>(
    account_to_close_info: &AccountInfo<'a>,
    funds_dest_account_info: &AccountInfo<'a>,
) -> ProgramResult {
    let rent = Rent::get()?;

    let account_size = account_to_close_info.data_len();
    let account_rent = rent.minimum_balance(account_size);
    let one_byte_rent = rent.minimum_balance(1);

    let amount_to_return = account_rent
        .checked_sub(one_byte_rent)
        .ok_or(MplCoreError::NumericalOverflowError)?;

    // Transfer lamports from the account to the destination account.
    let dest_starting_lamports = funds_dest_account_info.lamports();
    let new_dest_lamports = dest_starting_lamports
        .checked_add(amount_to_return)
        .ok_or(MplCoreError::NumericalOverflowError)?;
    **funds_dest_account_info.try_borrow_mut_lamports()? = new_dest_lamports;

    let close_starting_lamports = account_to_close_info.lamports();
    let new_close_lamports = close_starting_lamports
        .checked_sub(amount_to_return)
        .ok_or(MplCoreError::NumericalOverflowError)?;
    **account_to_close_info.try_borrow_mut_lamports()? = new_close_lamports;

    account_to_close_info.realloc(1, false)?;
    account_to_close_info.try_borrow_mut_data()?[0] = Key::Uninitialized.to_u8().unwrap();

    Ok(())
}

/// Resize an account using realloc and retain any lamport overages, modified from Solana Cookbook
pub(crate) fn resize_or_reallocate_account<'a>(
    target_account: &AccountInfo<'a>,
    funding_account: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    new_size: usize,
) -> ProgramResult {
    // If the account is already the correct size, return.
    if new_size == target_account.data_len() {
        return Ok(());
    }

    let rent = Rent::get()?;
    let new_minimum_balance = rent.minimum_balance(new_size);
    let current_minimum_balance = rent.minimum_balance(target_account.data_len());
    let account_infos = &[
        funding_account.clone(),
        target_account.clone(),
        system_program.clone(),
    ];

    if new_minimum_balance >= current_minimum_balance {
        let lamports_diff = new_minimum_balance.saturating_sub(current_minimum_balance);
        invoke(
            &system_instruction::transfer(funding_account.key, target_account.key, lamports_diff),
            account_infos,
        )?;
    } else {
        // return lamports to the compressor
        let lamports_diff = current_minimum_balance.saturating_sub(new_minimum_balance);

        let funding_starting_lamports = funding_account.lamports();
        let new_funding_lamports = funding_starting_lamports
            .checked_add(lamports_diff)
            .ok_or(MplCoreError::NumericalOverflowError)?;
        **funding_account.try_borrow_mut_lamports()? = new_funding_lamports;

        let target_starting_lamports = target_account.lamports();
        let new_target_lamports = target_starting_lamports
            .checked_sub(lamports_diff)
            .ok_or(MplCoreError::NumericalOverflowError)?;
        **target_account.try_borrow_mut_lamports()? = new_target_lamports;
    }

    target_account.realloc(new_size, false)?;

    Ok(())
}
