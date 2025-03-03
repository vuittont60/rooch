#[test_only]
/// This test module is used to test the coin store logic
module rooch_framework::coin_store_test{
    use std::signer;
    use std::string;
    use std::option;
    use moveos_std::context::{Context};
    use moveos_std::object;
    use rooch_framework::account;
    use rooch_framework::coin;
    use rooch_framework::account_coin_store;
    use rooch_framework::coin_store;

    #[test_only]
    struct FakeCoin has key, store {}

    #[test_only]
    fun register_fake_coin(
        ctx: &mut Context,
        decimals: u8,
    ) {
        coin::register_extend<FakeCoin>(
            ctx,
            string::utf8(b"Fake coin"),
            string::utf8(b"FCD"),
            decimals,
        );
    }

    #[test_only]
    fun mint_and_deposit(ctx: &mut Context,to_address: address, amount: u256) {
        let coins_minted = coin::mint_extend<FakeCoin>(ctx, amount);
        account_coin_store::deposit(ctx, to_address, coins_minted);
    }

    #[test_only]
    fun freeze_account_coin_store(
        ctx: &mut Context,
        addr: address,
        frozen: bool,
    ) {
        let coin_store_id_opt = account_coin_store::coin_store_id<FakeCoin>(ctx, addr);
        let coin_store_id = option::extract(&mut coin_store_id_opt);
        coin_store::freeze_coin_store_extend<FakeCoin>(ctx, coin_store_id, frozen);
    }

    #[test]
    public fun test_coin_store(){
        let ctx = rooch_framework::genesis::init_for_test();
        register_fake_coin(&mut ctx, 9);

        let coin_store_ref = coin_store::create_coin_store<FakeCoin>(&mut ctx);
        {
            let coin_store = object::borrow_mut(&mut coin_store_ref);

            let coin_minted = coin::mint_extend<FakeCoin>(&mut ctx, 100);
            coin_store::deposit(coin_store, coin_minted);

            assert!(coin_store::balance(coin_store) == 100, 1);

            let coin_withdrawn = coin_store::withdraw<FakeCoin>(coin_store, 10);

            assert!(coin::value(&coin_withdrawn) == 10, 2);
            assert!(coin_store::balance(coin_store) == 90, 3);
            coin::burn_extend(&mut ctx, coin_withdrawn);
        };
        let coin = coin_store::remove_coin_store<FakeCoin>(coin_store_ref);
        assert!(coin::value(&coin) == 90, 4);
        coin::burn_extend(&mut ctx, coin);
        assert!(coin::supply<FakeCoin>(&ctx) == 0, 5);

        moveos_std::context::drop_test_context(ctx);
    }

    #[test(account = @0x42)]
    public fun test_is_account_coin_store_frozen(account: signer) {
        let ctx = rooch_framework::genesis::init_for_test();
        let addr = signer::address_of(&account);
        
        register_fake_coin(&mut ctx, 9);

        // An non do_accept_coined account is has a no frozen coin store by default
        assert!(!account_coin_store::is_account_coin_store_frozen<FakeCoin>(&ctx, addr), 1);
        
        account::create_account_for_test(&mut ctx, addr);
        mint_and_deposit(&mut ctx, addr, 100);

        // freeze account
        freeze_account_coin_store(&mut ctx, addr, true);
        assert!(account_coin_store::is_account_coin_store_frozen<FakeCoin>(&ctx, addr), 2);

        // unfreeze account
        freeze_account_coin_store(&mut ctx, addr, false);
        assert!(!account_coin_store::is_account_coin_store_frozen<FakeCoin>(&ctx, addr), 3);

        moveos_std::context::drop_test_context(ctx);
    }

    #[test(account = @0x42)]
    #[expected_failure(abort_code = 327682, location = rooch_framework::coin_store)]
    fun test_withdraw_from_account_frozen(account: signer) {
        let ctx = rooch_framework::genesis::init_for_test();
        let account_addr = signer::address_of(&account);
        account::create_account_for_test(&mut ctx, account_addr);
        register_fake_coin(&mut ctx, 9);

        mint_and_deposit(&mut ctx, account_addr, 100);
        freeze_account_coin_store(&mut ctx, account_addr, true);
        let coin = account_coin_store::withdraw<FakeCoin>(&mut ctx, &account, 10);
        coin::burn_extend(&mut ctx, coin);

        moveos_std::context::drop_test_context(ctx);
    }

    #[test(account = @0x42)]
    #[expected_failure(abort_code = 327682, location = rooch_framework::coin_store)]
    fun test_deposit_to_account_frozen(account: signer) {
        let ctx = rooch_framework::genesis::init_for_test();
        let account_addr = signer::address_of(&account);
        account::create_account_for_test(&mut ctx, account_addr);

        register_fake_coin(&mut ctx, 9);

        mint_and_deposit(&mut ctx, account_addr, 100);
        let coins_minted = coin::mint_extend<FakeCoin>(&mut ctx, 100);
        freeze_account_coin_store(&mut ctx, account_addr, true);
        account_coin_store::deposit(&mut ctx, account_addr, coins_minted);

        moveos_std::context::drop_test_context(ctx);
    }

    #[test(account = @0x42)]
    fun test_deposit_widthdraw_unfrozen(account: signer) {
        let ctx = rooch_framework::genesis::init_for_test();
        let account_addr = signer::address_of(&account);

        account::create_account_for_test(&mut ctx, account_addr);
        register_fake_coin(&mut ctx, 9);
        mint_and_deposit(&mut ctx, account_addr, 100);

        let coins_minted = coin::mint_extend<FakeCoin>(&mut ctx, 100);
        freeze_account_coin_store(&mut ctx, account_addr, true);
        freeze_account_coin_store(&mut ctx, account_addr, false);
        account_coin_store::deposit(&mut ctx, account_addr, coins_minted);

        freeze_account_coin_store(&mut ctx, account_addr, true);
        freeze_account_coin_store(&mut ctx, account_addr, false);
        let coin = account_coin_store::withdraw<FakeCoin>(&mut ctx, &account, 10);
        coin::burn_extend(&mut ctx, coin);

        moveos_std::context::drop_test_context(ctx);
    }

}