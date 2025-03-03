// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

module nft::collection{
    use std::option;
    use std::option::Option;
    use std::string::{Self, String};
    use moveos_std::display;
    use moveos_std::object::{ObjectID};
    use moveos_std::event;
    use moveos_std::context::{Self, Context};
    use moveos_std::object::{Self, Object};

    friend nft::nft;

    const ErrorCollectionMaximumSupply: u64 = 1;

    struct Collection has key{
        name: String,
        creator: address,
        supply:  Supply,
    }

    struct Supply has store{
        current: u64,
        maximum: Option<u64>,
    }

    struct CreateCollectionEvent{
        object_id: ObjectID,
        name: String,
        creator: address,
        maximum: Option<u64>,
        description: String,
    }

    fun init(ctx: &mut Context){
        let collection_display_obj = display::object_display<Collection>(ctx); 
        display::set_value(collection_display_obj, string::utf8(b"name"), string::utf8(b"{ value.name }"));
        display::set_value(collection_display_obj, string::utf8(b"uri"), string::utf8(b"https:://base_url/{ id }"));
        display::set_value(collection_display_obj, string::utf8(b"description"), string::utf8(b"{ value.description }"));
        display::set_value(collection_display_obj, string::utf8(b"creator"), string::utf8(b"{ creator }"));
        display::set_value(collection_display_obj, string::utf8(b"supply"), string::utf8(b"{ value.supply }"));
    }

    /// Create a new collection Object
    public fun create_collection(
        ctx: &mut Context,
        name: String,
        creator: address,
        description: String,
        max_supply: Option<u64>,
    ) : Object<Collection> {

        let collection = Collection {
            name,
            creator,
            supply: Supply {
                current: 0,
                maximum: max_supply,
            },
        };

        let collection_obj = context::new_object(
            ctx,
            collection
        );
        event::emit(
            ctx,
            CreateCollectionEvent {
                object_id: object::id(&collection_obj),
                name,
                creator,
                maximum: max_supply,
                description,
            }
        );
        object::transfer_extend(&mut collection_obj, creator);
        collection_obj
    }

    public(friend) fun increment_supply(collection: &mut Collection): Option<u64>{
        collection.supply.current = collection.supply.current + 1;
        if(option::is_some(&collection.supply.maximum)){
            assert!(collection.supply.current <= *option::borrow(&collection.supply.maximum), ErrorCollectionMaximumSupply);
            option::some(collection.supply.current)
        }else{
            option::none<u64>()
        }
    }

    public(friend) fun decrement_supply(collection: &mut Collection): Option<u64>{
        collection.supply.current = collection.supply.current - 1;
        if(option::is_some(&collection.supply.maximum)){
            option::some(collection.supply.current)
        }else{
            option::none<u64>()
        }
    } 

    // view
    public fun name(collection: &Collection): String{
        collection.name
    }

    public fun creator(collection: &Collection): address{
        collection.creator
    }

    public fun current_supply(collection: &Collection): u64{
        collection.supply.current
    }

    public fun maximum_supply(collection: &Collection): Option<u64>{
        collection.supply.maximum
    }

}
