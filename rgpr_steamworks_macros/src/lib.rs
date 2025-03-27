use proc_macro::TokenStream;

pub(crate) mod callback;
pub(crate) mod sys;

/// Satanic macro for generating `CallbackRaw` and `Callback` trait implementations automatically.
/// This is made for usage _inside_ of `rgpr_steamworks`.
///
/// Does a lot:
/// - Only generates a `Callback` impl if the visibility is `pub`
/// - If the `keep;` token is provided, `Callback::KEEP_REGISTERED` will be set to `true`
/// - Automatically generates the `register` function, but can be overridden using the `new` token
/// - The `data` token generates the `on_callback` function, and allows specifying types  
///     which are automatically passed to the `Fn` type trait object and `call_listener` function
/// - Automatically fills `CType` based on the struct's identity
/// - Automatically fills `CALLBACK_ID` based on the struct's identity
/// - Automatic sys identities can be overridden with `sys FooBar;`
/// - Doing `new steam;` is a shortcut for writing `new steam { Self { steam: steam.child() } }`
#[proc_macro]
pub fn callback(token_stream: TokenStream) -> TokenStream {
	callback::callback(token_stream)
}


#[proc_macro]
pub fn extern_c_interface(token_stream: TokenStream) -> TokenStream {
	sys::extern_c_interface(token_stream)
}
