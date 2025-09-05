use cite_core::ui::SourceUi;
use cite_git::GitSource;
use proc_macro::TokenStream;
use syn::{parse_macro_input, Expr, Lit, Result};

#[proc_macro_attribute]
pub fn helper_macro_git(args: TokenStream, input: TokenStream) -> TokenStream {
	// Parse the arguments to extract the doc number
	let args = parse_macro_input!(args with syn::punctuated::Punctuated<Expr, syn::Token![,]>::parse_terminated);

	// Extract the doc type from the arguments
	let doc_type = match extract_doc_type(&args) {
		Ok(doc_type) => doc_type,
		Err(err) => return err.to_compile_error().into(),
	};

	// Extract the doc number from the arguments
	let doc_num = match extract_doc_number(&args) {
		Ok(num) => num,
		Err(err) => return err.to_compile_error().into(),
	};

	// Parse the input item
	let mut item = parse_macro_input!(input as syn::Item);

	// get the current commit hash
	let commit_hash = std::process::Command::new("git")
		.arg("rev-parse")
		.arg("HEAD")
		.output()
		.expect("Failed to get current commit hash");
	let commit_hash =
		String::from_utf8(commit_hash.stdout).expect("Failed to convert commit hash to string");

	// get the working directory of the build as would be used by cargo
	let working_dir = std::env::var("CARGO_MANIFEST_DIR").expect("Failed to get working directory");

	// zero pad the doc number to 000-000-000
	// e.g. 1 -> 000-000-001
	let doc_num = format!("{:09}", doc_num);
	// add the dashes
	let doc_num = format!("{}-{}-{}", doc_num[..3], doc_num[3..6], doc_num[6..]);

	// construct a path to search for something similar to the doc_num
	let path = format!("{}/{}/{}", working_dir, doc_type, doc_num);

	// find a link which begins with the path string and read the real path
	// TODO: this needs to be updated, something like globbing.
	let link_path = std::fs::read_link(path).expect("Failed to read link");

	// trim the working directory from the link path
	let link_path = link_path.trim_start_matches(&working_dir);

	// Create the actual GitSource directly
	let git_source = GitSource::try_new(
		"https://github.com/ramate-io/oac",
		&link_path,
		&commit_hash,
		"main",
		None,
	)
	.expect("Failed to create GitSource");

	// Use the SourceUi trait to generate the doc attribute
	let doc_attr = git_source.to_above_doc_attr().expect("Failed to generate doc attribute");
	let doc_comment = doc_attr.to_doc_attr_string();

	// Add the doc attribute to the item
	add_doc_attribute(&mut item, &doc_comment);

	// Return the modified item
	quote::quote!(#item).into()
}

/// Extract the doc number from the macro arguments
fn extract_doc_number(args: &syn::punctuated::Punctuated<Expr, syn::Token![,]>) -> Result<u32> {
	if args.len() != 1 {
		return Err(syn::Error::new(
			proc_macro2::Span::call_site(),
			"helper_macro_git expects exactly one argument: doc = <number>",
		));
	}

	let arg = &args[0];
	if let Expr::Assign(assign_expr) = arg {
		if let Expr::Path(left_path) = &*assign_expr.left {
			if left_path.path.segments.len() == 1 && left_path.path.segments[0].ident == "doc" {
				if let Expr::Lit(expr_lit) = &*assign_expr.right {
					if let Lit::Int(lit_int) = &expr_lit.lit {
						return lit_int.base10_parse::<u32>();
					}
				}
			}
		}
	}

	Err(syn::Error::new(proc_macro2::Span::call_site(), "helper_macro_git expects: doc = <number>"))
}

/// Add a doc attribute to the item
fn add_doc_attribute(item: &mut syn::Item, doc_content: &str) {
	let doc_attr = syn::parse_quote! {
		#[doc = #doc_content]
	};

	match item {
		syn::Item::Fn(item_fn) => {
			item_fn.attrs.insert(0, doc_attr);
		}
		syn::Item::Struct(item_struct) => {
			item_struct.attrs.insert(0, doc_attr);
		}
		syn::Item::Enum(item_enum) => {
			item_enum.attrs.insert(0, doc_attr);
		}
		syn::Item::Trait(item_trait) => {
			item_trait.attrs.insert(0, doc_attr);
		}
		syn::Item::Impl(item_impl) => {
			item_impl.attrs.insert(0, doc_attr);
		}
		syn::Item::Mod(item_mod) => {
			item_mod.attrs.insert(0, doc_attr);
		}
		_ => {
			// For other item types, we'll just ignore them
		}
	}
}
