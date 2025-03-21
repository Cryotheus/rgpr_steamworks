use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::Semi;
use syn::{
	Block, Error, Expr, ExprCall, ExprClosure, ExprField, Index, ItemStruct, Member, Pat, PathArguments, ReturnType, Token, Type, TypeParamBound, TypeTraitObject, TypeTuple, Visibility,
	parse_macro_input, parse_quote,
};

struct CallbackMacro {
	block_data: Block,
	block_new: Block,
	listeners: bool,
	item_struct: ItemStruct,
	keep_registered: bool,
	output_type: Type,

	data_ident: Option<Ident>,
	steam_ident: Option<Ident>,
	sys_ident: Option<Ident>,
}

impl Parse for CallbackMacro {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let item_struct = input.parse::<ItemStruct>()?;
		let mut callback_macro = CallbackMacro {
			block_data: Block {
				brace_token: Default::default(),
				stmts: Vec::new(),
			},

			block_new: syn::parse(quote! { { Self } }.into())?,

			listeners: match item_struct.vis {
				Visibility::Public(_) => true,
				_ => false,
			},

			item_struct,
			keep_registered: false,

			output_type: Type::Tuple(TypeTuple {
				paren_token: Default::default(),
				elems: Default::default(),
			}),

			data_ident: None,
			steam_ident: None,
			sys_ident: None,
		};

		loop {
			let lookahead = input.lookahead1();

			if lookahead.peek(syn::token::Or) && false {
				let closure = input.parse::<ExprClosure>()?;
				let first_input = closure.inputs.get(0).ok_or(Error::new(closure.span(), "malformed data closure"))?;

				let Pat::Type(pat_type) = first_input else {
					return Err(Error::new(closure.span(), "malformed data closure input"));
				};

				let Pat::Ident(pat_ident) = pat_type.pat.as_ref() else {
					return Err(Error::new(closure.span(), "malformed data closure input ident"));
				};

				if pat_ident.ident.to_string().as_str() != "data" {
					return Err(Error::new(closure.span(), "data closure named something other than data"));
				}

				if let ReturnType::Type(_, boxed_type) = closure.output {
					callback_macro.output_type = *boxed_type;
				}

				let body = closure.body;
				callback_macro.block_data = parse_quote! { #body };

				continue;
			} else if !lookahead.peek(syn::Ident) {
				break;
			}

			let ident = input.parse::<Ident>()?;

			match ident.to_string().as_str() {
				"data" => {
					let lookahead = input.lookahead1();

					if lookahead.peek(Token![->]) {
						if let ReturnType::Type(_, boxed_type) = input.parse::<ReturnType>()? {
							callback_macro.output_type = *boxed_type;
						}
					}

					callback_macro.block_data = input.parse::<Block>()?;
					callback_macro.data_ident = Some(ident);
				}

				"keep" => {
					callback_macro.keep_registered = true;

					input.parse::<Semi>()?;
				}

				"new" => {
					if input.lookahead1().peek(Ident::peek_any) {
						let steam_ident = input.parse::<Ident>()?;

						if steam_ident.to_string().as_str() != "steam" {
							return Err(Error::new(steam_ident.span(), "optional token after new must be named `steam`"));
						}

						callback_macro.steam_ident = Some(steam_ident.clone());

						//check if they just want the steam child in their struct
						if input.lookahead1().peek(Semi) {
							input.parse::<Semi>()?;

							callback_macro.block_new = parse_quote! { { Self { steam: #steam_ident.child() } } };

							continue;
						}
					}

					callback_macro.block_new = input.parse::<Block>()?;
				}

				"sys" => {
					let sys_ident = input.parse::<Ident>()?;
					callback_macro.sys_ident = Some(sys_ident);

					input.parse::<Semi>()?;
				}

				_ => return Err(Error::new(ident.span(), "unknown macro instruction")),
			}
		}

		Ok(callback_macro)
	}
}

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
	let callback_macro = parse_macro_input!(token_stream as CallbackMacro);
	let item_struct = &callback_macro.item_struct;
	let ident = &item_struct.ident;

	let sys_ident = match callback_macro.sys_ident {
		None => Ident::new(&format!("{ident}_t"), ident.span()),
		Some(specified) => syn::parse_str::<Ident>(&format!("{specified}_t")).expect("bad sys token identity"),
	};

	let sys_ident_string = sys_ident.to_string();
	let sys_ident_index = Ident::new(&format!("{sys_ident}_k_iCallback"), sys_ident.span());

	let block_data = callback_macro.block_data;
	let block_new = callback_macro.block_new;
	let output_type = callback_macro.output_type;
	let data_ident = callback_macro.data_ident.unwrap_or(parse_quote!(_data));
	let steam_ident = callback_macro.steam_ident.unwrap_or(parse_quote!(_steam));

	//token stream for impl Callback
	//if we're doing that
	let mut listener_impl = proc_macro2::TokenStream::new();

	if callback_macro.listeners {
		let mut listener_fn_call: ExprCall = parse_quote! { listener_fn() };

		//create the fn trait object
		let mut trait_object: TypeTraitObject = parse_quote! { dyn FnMut() + Send + Sync };

		//get a &mut to the list of input types for the FnMut(...)
		let TypeParamBound::Trait(trait_bound) = trait_object.bounds.get_mut(0).unwrap() else { panic!() };
		let PathArguments::Parenthesized(path_args) = &mut trait_bound.path.segments.get_mut(0).unwrap().arguments else {
			panic!()
		};

		let trait_object_inputs = &mut path_args.inputs;

		if let Type::Tuple(tuple) = &output_type {
			//if it's a tuple type
			//we should handle each field individually
			for (index, tuple_elem) in tuple.elems.iter().enumerate() {
				let mut expr_field: ExprField = parse_quote! { params.0 };
				expr_field.member = Member::Unnamed(Index::from(index));

				trait_object_inputs.push(tuple_elem.clone());
				listener_fn_call.args.push(Expr::Field(expr_field));
			}
		} else {
			//otherwise, pass the entire parameter without unpacking the tuple
			let ident: Expr = parse_quote! { params };

			listener_fn_call.args.push(ident);
			trait_object_inputs.push(output_type.clone());
		}

		let mut keep_registered = proc_macro2::TokenStream::new();

		if callback_macro.keep_registered {
			keep_registered = quote! { const KEEP_REGISTERED: bool = true; };
		}

		//fill in the tokens for listener_impl
		//if we want to generate a Callback impl
		listener_impl = quote! {
			impl crate::call::Callback for #ident {
				#keep_registered
				type Fn = #trait_object;

				fn call_listener(&mut self, listener_fn: &mut Self::Fn, params: Self::Output, _: crate::Private) {
					#listener_fn_call;
				}
			}
		};
	}

	//TODO: automatically generate example listener functions
	let quoted = quote! {
		#[doc = "**Callback**"]
		#[doc = ""]
		#[doc = "Listener function signature:"]
		#[doc = "`TODO!`"]
		#[doc(alias = #sys_ident_string)]
		#[derive(Debug)]
		#item_struct

		unsafe impl crate::call::CallbackRaw for #ident {
			const CALLBACK_ID: i32 = crate::sys::#sys_ident_index as i32;
			type CType = crate::sys::#sys_ident;
			type Output = #output_type;

			unsafe fn on_callback(&mut self, #data_ident: &Self::CType, _: crate::Private) -> Self::Output #block_data

			fn register(#steam_ident: &crate::interfaces::SteamInterface, _: crate::Private) -> Self #block_new
		}

		#listener_impl
	};

	quoted.into()
}
