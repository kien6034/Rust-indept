pub mod smart_pointers;
pub mod generic_trait_lifetime;
pub mod types;
pub mod macros;

fn main() {
    //smart_pointers::smart_pointer::run();
    //smart_pointers::dereferrence::run();
    //smart_pointers::rc::run();
    //generic_trait_lifetime::generic::run();
    //generic_trait_lifetime::trait_example::run();
    //generic_trait_lifetime::lifetime::run();
    //types::phantomdata::run();
    //types::typed_key_pattern::run();

    macros::declarative_macros::run();
}
