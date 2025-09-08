use cite_core::ui::SourceUi;
use cite_git::GitSource;
use proc_macro::TokenStream;
use syn::{parse_macro_input, Expr, Lit, Result};

/// Find the OAC repository root by looking for the artifact-types file
fn find_oac_root(start_dir: &str) -> Option<String> {
	let mut current_dir = std::path::Path::new(start_dir);

	loop {
		let artifact_types_path = current_dir.join("artifact-types");
		if artifact_types_path.exists() {
			return Some(current_dir.to_string_lossy().to_string());
		}

		// Move up one directory
		match current_dir.parent() {
			Some(parent) => current_dir = parent,
			None => return None,
		}
	}
}

#[proc_macro_attribute]
pub fn oac(args: TokenStream, input: TokenStream) -> TokenStream {
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
	let manifest_dir =
		std::env::var("CARGO_MANIFEST_DIR").expect("Failed to get working directory");

	// Find the OAC repository root (where artifact-types file exists)
	let working_dir = find_oac_root(&manifest_dir).expect("Failed to find OAC repository root");

	// zero pad the doc number to 000-000-000
	// e.g. 1 -> 000-000-001
	let doc_num_padded = format!("{:09}", doc_num);
	// add the dashes
	let doc_num =
		format!("{}-{}-{}", &doc_num_padded[..3], &doc_num_padded[3..6], &doc_num_padded[6..]);

	// construct the artifact name based on the pattern: {type}-{padded_number}
	let artifact_name = format!("{}-{}", doc_type, doc_num);

	// try to find the artifact using the symlink structure we created
	let symlink_path = format!("{}/{}/{}", working_dir, doc_type, artifact_name);

	// read the symlink to get the actual path
	let link_path = std::fs::read_link(&symlink_path)
		.map_err(|_| format!("Failed to find artifact symlink at: {}", symlink_path))
		.expect("Failed to read artifact symlink");

	// convert to absolute path and then make relative to working directory
	let absolute_link_path = if link_path.is_absolute() {
		link_path
	} else {
		// resolve relative symlink path
		let symlink_dir = std::path::Path::new(&symlink_path).parent().unwrap();
		symlink_dir
			.join(&link_path)
			.canonicalize()
			.expect("Failed to resolve symlink path")
	};

	// make relative to working directory
	let link_path = absolute_link_path
		.strip_prefix(&working_dir)
		.expect("Link path is not within working directory")
		.to_string_lossy()
		.to_string();

	// Create the actual GitSource directly
	let git_source = GitSource::try_new(
		"https://github.com/ramate-io/oac",
		&format!("{}/", link_path), // add a trailing slash to the link path
		&commit_hash,
		"main",
		// use the caps(artifact-type)-doc as the name
		Some(format!("{}-{}", doc_type.to_uppercase(), doc_num)),
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

/// Extract the doc type from the macro arguments and validate against artifact-types
fn extract_doc_type(args: &syn::punctuated::Punctuated<Expr, syn::Token![,]>) -> Result<String> {
	if args.len() != 2 {
		return Err(syn::Error::new(
			proc_macro2::Span::call_site(),
			"oac expects exactly two arguments: artifact_type, number",
		));
	}

	let arg = &args[0];
	if let Expr::Path(path) = arg {
		if path.path.segments.len() == 1 {
			let artifact_type = path.path.segments[0].ident.to_string();

			// Validate against artifact-types file
			if !is_valid_artifact_type(&artifact_type) {
				return Err(syn::Error::new(
					proc_macro2::Span::call_site(),
					format!("Invalid artifact type '{}'. Must be one of the types listed in artifact-types file", artifact_type),
				));
			}

			return Ok(artifact_type);
		}
	}

	Err(syn::Error::new(
		proc_macro2::Span::call_site(),
		"oac expects first argument to be an artifact type identifier",
	))
}

/// Extract the doc number from the macro arguments
fn extract_doc_number(args: &syn::punctuated::Punctuated<Expr, syn::Token![,]>) -> Result<u32> {
	if args.len() != 2 {
		return Err(syn::Error::new(
			proc_macro2::Span::call_site(),
			"oac expects exactly two arguments: artifact_type, number",
		));
	}

	let arg = &args[1];
	if let Expr::Lit(expr_lit) = arg {
		if let Lit::Int(lit_int) = &expr_lit.lit {
			return lit_int.base10_parse::<u32>();
		}
	}

	Err(syn::Error::new(
		proc_macro2::Span::call_site(),
		"oac expects second argument to be a number",
	))
}

/// Check if the artifact type is valid by reading the artifact-types file
fn is_valid_artifact_type(artifact_type: &str) -> bool {
	// Get the manifest directory and find OAC root
	let manifest_dir = match std::env::var("CARGO_MANIFEST_DIR") {
		Ok(dir) => dir,
		Err(_) => return false,
	};

	let oac_root = match find_oac_root(&manifest_dir) {
		Some(root) => root,
		None => return false,
	};

	// Read the artifact-types file
	let artifact_types_path = format!("{}/artifact-types", oac_root);
	let artifact_types_content = match std::fs::read_to_string(&artifact_types_path) {
		Ok(content) => content,
		Err(_) => return false,
	};

	// Parse the artifact types (one per line, ignore comments and empty lines)
	for line in artifact_types_content.lines() {
		let line = line.trim();
		if line.is_empty() || line.starts_with('#') {
			continue;
		}
		if line == artifact_type {
			return true;
		}
	}

	false
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
