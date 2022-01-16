use crate::action::ActionError::{
    ArithmeticOverflow, LockedAccount, NegativeAmountNotSupported,
    NoAmountAssociatedWithTransaction,
};
use crate::Account;
use crate::{Transaction, TransactionType};
use rust_decimal::Decimal;

pub type ActionResult = Result<(), ActionError>;
#[derive(Debug)]
pub enum ActionError {
    NegativeAmountNotSupported,
    NoAmountAssociatedWithTransaction,
    ArithmeticOverflow,
    LockedAccount,
}

pub trait Action {
    fn perform(&self, acc: &mut Account) -> ActionResult;
}

impl Action for Transaction {
    fn perform(&self, acc: &mut Account) -> ActionResult {
        if acc.balance.locked {
            // Short-circuit evaluation for locked accounts
            return Err(LockedAccount);
        }

        if let Some(true) = self.amount.map(|a| a.is_sign_negative()) {
            return Err(NegativeAmountNotSupported);
        }

        match self.ttype {
            TransactionType::Deposit => {
                let amount = self.amount.unwrap_or(Decimal::ZERO).round_dp(4);

                acc.balance.available = Self::add_amount(acc.balance.available, amount)?;
                acc.balance.total = Self::add_amount(acc.balance.total, amount)?;

                acc.transactions.entry(self.id).or_insert_with(|| *self);
            }

            TransactionType::Withdrawal => {
                let amount = self.amount.unwrap_or(Decimal::ZERO).round_dp(4);

                acc.balance.available = Self::subtract_amount(acc.balance.available, amount)?;
                acc.balance.total = Self::subtract_amount(acc.balance.total, amount)?;

                acc.transactions.entry(self.id).or_insert_with(|| *self);
            }

            TransactionType::Dispute => {
                if let (Some(tx), None) =
                    (acc.transactions.get(&self.id), acc.disputes.get(&self.id))
                {
                    acc.disputes.entry(self.id).or_insert_with(|| *tx);

                    let amount = tx
                        .amount
                        .ok_or(NoAmountAssociatedWithTransaction)?
                        .round_dp(4);

                    acc.balance.available = Self::subtract_amount(acc.balance.available, amount)?;
                    acc.balance.held = Self::add_amount(acc.balance.held, amount)?;
                }
            }

            TransactionType::Resolve => {
                if let (Some(tx), Some(_)) =
                    (acc.transactions.get(&self.id), acc.disputes.get(&self.id))
                {
                    let amount = tx
                        .amount
                        .ok_or(NoAmountAssociatedWithTransaction)?
                        .round_dp(4);

                    acc.balance.held = Self::subtract_amount(acc.balance.held, amount)?;
                    acc.balance.available = Self::add_amount(acc.balance.available, amount)?;

                    acc.disputes.remove_entry(&tx.id);
                }
            }

            TransactionType::Chargeback => {
                if let (Some(tx), Some(_)) =
                    (acc.transactions.get(&self.id), acc.disputes.get(&self.id))
                {
                    let amount = tx
                        .amount
                        .ok_or(NoAmountAssociatedWithTransaction)?
                        .round_dp(4);

                    acc.balance.held = Self::subtract_amount(acc.balance.held, amount)?;
                    acc.balance.total = Self::subtract_amount(acc.balance.total, amount)?;
                    acc.balance.locked = true;

                    acc.disputes.remove_entry(&tx.id);
                }
            }
        }

        Ok(())
    }
}

impl Transaction {
    fn add_amount(target: Decimal, amount: Decimal) -> Result<Decimal, ActionError> {
        // Since amount is guaranteed to be non-negative we do not need to check here.
        target.checked_add(amount).ok_or(ArithmeticOverflow)
    }

    fn subtract_amount(target: Decimal, amount: Decimal) -> Result<Decimal, ActionError> {
        Ok(
            match target.checked_sub(amount).ok_or(ArithmeticOverflow)? {
                res if res.is_sign_negative() => target,
                res => res,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
    use std::str::FromStr;

    #[test]
    fn test() {
        let test = Decimal::from_str("1.0").unwrap();
        let test = test.round_dp(4);
        assert_eq!(test, Decimal::ONE);

        let test = Decimal::from_str("1.5").unwrap();
        let test = test.round_dp(4);
        assert_eq!(test, Decimal::from_f32(1.5).unwrap());

        let test = Decimal::from_str("1.5093").unwrap();
        let test = test.round_dp(4);
        assert_eq!(test, Decimal::from_f32(1.5093).unwrap());

        let test = Decimal::from_str("5.158254234234").unwrap();
        let test = test.round_dp(4);
        assert_eq!(test.to_f64().unwrap(), 5.1583);

        let test = Decimal::from_str("-1").unwrap();
        let test = test.round_dp(4);
        assert_eq!(test.to_f64().unwrap(), -1.);
    }

    #[test]
    fn test_checked_add() {
        assert_eq!(
            Decimal::from_f32(1.5)
                .unwrap()
                .checked_add(Decimal::from_f32(1.5).unwrap())
                .unwrap()
                .to_f64()
                .unwrap(),
            3.0
        )
    }
}
