use rust_exercice::dto::client_account::ClientAccount;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit() {
        let mut account = ClientAccount::default();
        account.deposit(100.0);

        assert_eq!(account.available, 100.0);
        assert_eq!(account.total, 100.0);
        assert_eq!(account.held, 0.0);
        assert_eq!(account.locked, false);
    }

    #[test]
    fn test_withdrawal_success() {
        let mut account = ClientAccount::default();
        account.deposit(100.0);
        let result = account.withdrawal(50.0);

        assert!(result);
        assert_eq!(account.available, 50.0);
        assert_eq!(account.total, 50.0);
        assert_eq!(account.held, 0.0);
        assert_eq!(account.locked, false);
    }

    #[test]
    fn test_withdrawal_failure() {
        let mut account = ClientAccount::default();
        account.deposit(100.0);
        let result = account.withdrawal(150.0);

        assert!(!result);
        assert_eq!(account.available, 100.0);
        assert_eq!(account.total, 100.0);
        assert_eq!(account.held, 0.0);
        assert_eq!(account.locked, false);
    }

    #[test]
    fn test_dispute() {
        let mut account = ClientAccount::default();
        account.deposit(100.0);
        account.dispute(50.0);

        assert_eq!(account.available, 50.0);
        assert_eq!(account.held, 50.0);
        assert_eq!(account.total, 100.0);
        assert_eq!(account.locked, false);
    }

    #[test]
    fn test_resolve() {
        let mut account = ClientAccount::default();
        account.deposit(100.0);
        account.dispute(50.0);
        account.resolve(50.0);

        assert_eq!(account.available, 100.0);
        assert_eq!(account.held, 0.0);
        assert_eq!(account.total, 100.0);
        assert_eq!(account.locked, false);
    }

    #[test]
    fn test_chargeback() {
        let mut account = ClientAccount::default();
        account.deposit(100.0);
        account.dispute(50.0);
        account.chargeback(50.0);

        assert_eq!(account.available, 50.0);
        assert_eq!(account.held, 0.0);
        assert_eq!(account.total, 50.0);
        assert_eq!(account.locked, true);
    }
}
