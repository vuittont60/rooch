
<a name="0x2_event"></a>

# Module `0x2::event`

<code><a href="event.md#0x2_event_EventHandle">EventHandle</a></code>s with unique event handle id (GUID). It contains a counter for the number
of <code><a href="event.md#0x2_event_EventHandle">EventHandle</a></code>s it generates. An <code><a href="event.md#0x2_event_EventHandle">EventHandle</a></code> is used to count the number of
events emitted to a handle and emit events to the event store.


-  [Resource `EventHandle`](#0x2_event_EventHandle)
-  [Function `derive_event_handle_id`](#0x2_event_derive_event_handle_id)
-  [Function `get_event_handle`](#0x2_event_get_event_handle)
-  [Function `ensure_event_handle`](#0x2_event_ensure_event_handle)
-  [Function `emit`](#0x2_event_emit)


<pre><code><b>use</b> <a href="">0x1::hash</a>;
<b>use</b> <a href="bcs.md#0x2_bcs">0x2::bcs</a>;
<b>use</b> <a href="context.md#0x2_context">0x2::context</a>;
<b>use</b> <a href="object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="type_info.md#0x2_type_info">0x2::type_info</a>;
</code></pre>



<a name="0x2_event_EventHandle"></a>

## Resource `EventHandle`

A handle for an event such that:
1. Other modules can emit events to this handle.
2. Storage can use this handle to prove the total number of events that happened in the past.


<pre><code><b>struct</b> <a href="event.md#0x2_event_EventHandle">EventHandle</a> <b>has</b> key
</code></pre>



<a name="0x2_event_derive_event_handle_id"></a>

## Function `derive_event_handle_id`

A globally unique ID for this event stream. event handler id equal to guid.


<pre><code><b>public</b> <b>fun</b> <a href="event.md#0x2_event_derive_event_handle_id">derive_event_handle_id</a>&lt;T&gt;(): <a href="object.md#0x2_object_ObjectID">object::ObjectID</a>
</code></pre>



<a name="0x2_event_get_event_handle"></a>

## Function `get_event_handle`

Method to get event handle Metadata
If event_handle_id doesn't exist, sender will be default address 0x0


<pre><code><b>public</b> <b>fun</b> <a href="event.md#0x2_event_get_event_handle">get_event_handle</a>&lt;T&gt;(ctx: &<a href="context.md#0x2_context_Context">context::Context</a>): (<a href="object.md#0x2_object_ObjectID">object::ObjectID</a>, <b>address</b>, u64)
</code></pre>



<a name="0x2_event_ensure_event_handle"></a>

## Function `ensure_event_handle`



<pre><code><b>public</b> <b>fun</b> <a href="event.md#0x2_event_ensure_event_handle">ensure_event_handle</a>&lt;T&gt;(ctx: &<b>mut</b> <a href="context.md#0x2_context_Context">context::Context</a>)
</code></pre>



<a name="0x2_event_emit"></a>

## Function `emit`

Emit a custom Move event, sending the data offchain.

Used for creating custom indexes and tracking onchain
activity in a way that suits a specific application the most.

The type T is the main way to index the event, and can contain
phantom parameters, eg. emit(MyEvent<phantom T>).


<pre><code><b>public</b> <b>fun</b> <a href="event.md#0x2_event_emit">emit</a>&lt;T&gt;(ctx: &<b>mut</b> <a href="context.md#0x2_context_Context">context::Context</a>, <a href="event.md#0x2_event">event</a>: T)
</code></pre>
