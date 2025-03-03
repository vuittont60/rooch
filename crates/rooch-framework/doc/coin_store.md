
<a name="0x3_coin_store"></a>

# Module `0x3::coin_store`



-  [Struct `Balance`](#0x3_coin_store_Balance)
-  [Resource `CoinStore`](#0x3_coin_store_CoinStore)
-  [Constants](#@Constants_0)
-  [Function `create_coin_store`](#0x3_coin_store_create_coin_store)
-  [Function `create_coin_store_extend`](#0x3_coin_store_create_coin_store_extend)
-  [Function `remove_coin_store`](#0x3_coin_store_remove_coin_store)
-  [Function `coin_type`](#0x3_coin_store_coin_type)
-  [Function `balance`](#0x3_coin_store_balance)
-  [Function `is_frozen`](#0x3_coin_store_is_frozen)
-  [Function `withdraw`](#0x3_coin_store_withdraw)
-  [Function `deposit`](#0x3_coin_store_deposit)
-  [Function `freeze_coin_store_extend`](#0x3_coin_store_freeze_coin_store_extend)
-  [Function `create_coin_store_internal`](#0x3_coin_store_create_coin_store_internal)
-  [Function `transfer`](#0x3_coin_store_transfer)


<pre><code><b>use</b> <a href="">0x1::error</a>;
<b>use</b> <a href="">0x1::string</a>;
<b>use</b> <a href="">0x2::context</a>;
<b>use</b> <a href="">0x2::object</a>;
<b>use</b> <a href="">0x2::type_info</a>;
<b>use</b> <a href="coin.md#0x3_coin">0x3::coin</a>;
</code></pre>



<a name="0x3_coin_store_Balance"></a>

## Struct `Balance`

The Balance resource that stores the balance of a specific coin type.


<pre><code><b>struct</b> <a href="coin_store.md#0x3_coin_store_Balance">Balance</a> <b>has</b> store
</code></pre>



<a name="0x3_coin_store_CoinStore"></a>

## Resource `CoinStore`

A holder of a specific coin types.
These are kept in a single resource to ensure locality of data.


<pre><code><b>struct</b> <a href="coin_store.md#0x3_coin_store_CoinStore">CoinStore</a> <b>has</b> key
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="0x3_coin_store_ErrorInSufficientBalance"></a>

Not enough balance to withdraw from CoinStore


<pre><code><b>const</b> <a href="coin_store.md#0x3_coin_store_ErrorInSufficientBalance">ErrorInSufficientBalance</a>: u64 = 4;
</code></pre>



<a name="0x3_coin_store_ErrorCoinStoreIsFrozen"></a>

CoinStore is frozen. Coins cannot be deposited or withdrawn


<pre><code><b>const</b> <a href="coin_store.md#0x3_coin_store_ErrorCoinStoreIsFrozen">ErrorCoinStoreIsFrozen</a>: u64 = 2;
</code></pre>



<a name="0x3_coin_store_ErrorCoinStoreNotFound"></a>

The CoinStore is not found in the global object store


<pre><code><b>const</b> <a href="coin_store.md#0x3_coin_store_ErrorCoinStoreNotFound">ErrorCoinStoreNotFound</a>: u64 = 1;
</code></pre>



<a name="0x3_coin_store_ErrorCoinTypeAndStoreMismatch"></a>

The CoinType parameter and CoinType in CoinStore do not match


<pre><code><b>const</b> <a href="coin_store.md#0x3_coin_store_ErrorCoinTypeAndStoreMismatch">ErrorCoinTypeAndStoreMismatch</a>: u64 = 3;
</code></pre>



<a name="0x3_coin_store_create_coin_store"></a>

## Function `create_coin_store`

Create a new CoinStore Object for <code>CoinType</code> and return the Object
Anyone can create a CoinStore Object for public Coin<CoinType>, the <code>CoinType</code> must has <code>key</code> and <code>store</code> ability


<pre><code><b>public</b> <b>fun</b> <a href="coin_store.md#0x3_coin_store_create_coin_store">create_coin_store</a>&lt;CoinType: store, key&gt;(ctx: &<b>mut</b> <a href="_Context">context::Context</a>): <a href="_Object">object::Object</a>&lt;<a href="coin_store.md#0x3_coin_store_CoinStore">coin_store::CoinStore</a>&gt;
</code></pre>



<a name="0x3_coin_store_create_coin_store_extend"></a>

## Function `create_coin_store_extend`

This function is for the <code>CoinType</code> module to extend


<pre><code><b>public</b> <b>fun</b> <a href="coin_store.md#0x3_coin_store_create_coin_store_extend">create_coin_store_extend</a>&lt;CoinType: key&gt;(ctx: &<b>mut</b> <a href="_Context">context::Context</a>): <a href="_Object">object::Object</a>&lt;<a href="coin_store.md#0x3_coin_store_CoinStore">coin_store::CoinStore</a>&gt;
</code></pre>



<a name="0x3_coin_store_remove_coin_store"></a>

## Function `remove_coin_store`

Remove the CoinStore Object, return the Coin<T> in balance


<pre><code><b>public</b> <b>fun</b> <a href="coin_store.md#0x3_coin_store_remove_coin_store">remove_coin_store</a>&lt;CoinType: key&gt;(coin_store_object: <a href="_Object">object::Object</a>&lt;<a href="coin_store.md#0x3_coin_store_CoinStore">coin_store::CoinStore</a>&gt;): <a href="coin.md#0x3_coin_Coin">coin::Coin</a>&lt;CoinType&gt;
</code></pre>



<a name="0x3_coin_store_coin_type"></a>

## Function `coin_type`



<pre><code><b>public</b> <b>fun</b> <a href="coin_store.md#0x3_coin_store_coin_type">coin_type</a>(self: &<a href="coin_store.md#0x3_coin_store_CoinStore">coin_store::CoinStore</a>): <a href="_String">string::String</a>
</code></pre>



<a name="0x3_coin_store_balance"></a>

## Function `balance`



<pre><code><b>public</b> <b>fun</b> <a href="coin_store.md#0x3_coin_store_balance">balance</a>(self: &<a href="coin_store.md#0x3_coin_store_CoinStore">coin_store::CoinStore</a>): u256
</code></pre>



<a name="0x3_coin_store_is_frozen"></a>

## Function `is_frozen`



<pre><code><b>public</b> <b>fun</b> <a href="coin_store.md#0x3_coin_store_is_frozen">is_frozen</a>(self: &<a href="coin_store.md#0x3_coin_store_CoinStore">coin_store::CoinStore</a>): bool
</code></pre>



<a name="0x3_coin_store_withdraw"></a>

## Function `withdraw`

Withdraw <code>amount</code> Coin<CoinType> from the balance of the passed-in <code><a href="coin_store.md#0x3_coin_store">coin_store</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="coin_store.md#0x3_coin_store_withdraw">withdraw</a>&lt;CoinType: key&gt;(<a href="coin_store.md#0x3_coin_store">coin_store</a>: &<b>mut</b> <a href="coin_store.md#0x3_coin_store_CoinStore">coin_store::CoinStore</a>, amount: u256): <a href="coin.md#0x3_coin_Coin">coin::Coin</a>&lt;CoinType&gt;
</code></pre>



<a name="0x3_coin_store_deposit"></a>

## Function `deposit`

Deposit <code>amount</code> Coin<CoinType> to the balance of the passed-in <code><a href="coin_store.md#0x3_coin_store">coin_store</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="coin_store.md#0x3_coin_store_deposit">deposit</a>&lt;CoinType: key&gt;(<a href="coin_store.md#0x3_coin_store">coin_store</a>: &<b>mut</b> <a href="coin_store.md#0x3_coin_store_CoinStore">coin_store::CoinStore</a>, <a href="coin.md#0x3_coin">coin</a>: <a href="coin.md#0x3_coin_Coin">coin::Coin</a>&lt;CoinType&gt;)
</code></pre>



<a name="0x3_coin_store_freeze_coin_store_extend"></a>

## Function `freeze_coin_store_extend`

Freeze or Unfreeze a CoinStore to prevent withdraw and desposit
This function is for he <code>CoinType</code> module to extend,
Only the <code>CoinType</code> module can freeze or unfreeze a CoinStore by the coin store id


<pre><code><b>public</b> <b>fun</b> <a href="coin_store.md#0x3_coin_store_freeze_coin_store_extend">freeze_coin_store_extend</a>&lt;CoinType: key&gt;(ctx: &<b>mut</b> <a href="_Context">context::Context</a>, coin_store_id: <a href="_ObjectID">object::ObjectID</a>, frozen: bool)
</code></pre>



<a name="0x3_coin_store_create_coin_store_internal"></a>

## Function `create_coin_store_internal`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="coin_store.md#0x3_coin_store_create_coin_store_internal">create_coin_store_internal</a>&lt;CoinType: key&gt;(ctx: &<b>mut</b> <a href="_Context">context::Context</a>): <a href="_Object">object::Object</a>&lt;<a href="coin_store.md#0x3_coin_store_CoinStore">coin_store::CoinStore</a>&gt;
</code></pre>



<a name="0x3_coin_store_transfer"></a>

## Function `transfer`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="transfer.md#0x3_transfer">transfer</a>(coin_store_obj: &<b>mut</b> <a href="_Object">object::Object</a>&lt;<a href="coin_store.md#0x3_coin_store_CoinStore">coin_store::CoinStore</a>&gt;, owner: <b>address</b>)
</code></pre>
