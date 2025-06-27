use std::env;
use std::fs;
use std::time::Instant;
use syn;
use syn::{Item, ItemFn, ItemMod};

fn main() {
    let mut dir = env::current_dir().expect("Cannot access current working directory.");
    dir.push("cargolib");
    dir.push("src");
    dir.push("main.rs");
    let file_name = dir.as_path();
    let src = fs::read_to_string(&file_name).expect("unable to read file");
    let start_time = Instant::now();
    logic(&src);
    let elapsed_time = start_time.elapsed();
    println!("Execution took: {} ms", elapsed_time.as_millis());
}

fn logic(src: &str) {
    let syntax = syn::parse_file(&src).expect("unable to parse file");
    let items = syntax.items;
    let mut test_fns: Vec<&ItemFn> = Vec::new();
    println!("Working on [{}] outer level items", items.len());
    for item in &items {
        let maybe_test_fn = extract_test_fn(item);
        if let Some(test_fn) = maybe_test_fn {
            test_fns.push(test_fn);
        } else if let Item::Mod(item_mod) = item {
            extract_mod(item_mod, &mut test_fns);
        }
    }
    for test_fn in test_fns {
        println!("Found test function [{}]", test_fn.sig.ident);
    }

}

fn extract_mod<'a>(item_mod: &'a ItemMod, test_fns: &mut Vec<&'a ItemFn>) {
    if let Some(items_in_module) = &item_mod.content {
        for item in &items_in_module.1 {
            let maybe_test_fn = extract_test_fn(item);
            if let Some(test_fn) = maybe_test_fn {
                test_fns.push(test_fn);
            } else if let Item::Mod(inner_item_mod) = item {
                extract_mod(inner_item_mod, test_fns);
            }
        }
    }
}

fn extract_test_fn(item: &Item) -> Option<&ItemFn> {
    if let Item::Fn(item_fn) = item {
        let is_test_function = item_fn
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("test"));

        if !is_test_function {
            return None;
        }
        Some(item_fn)
        // println!("--- Documentation Comments ---");

        // Iterate over its attributes to find documentation comments
        // for attr in &item_fn.attrs {
        //     // Documentation comments are represented as #[doc = "comment content"]
        //     if attr.path().is_ident("doc") {
        //         // CORRECTED LINE: Call .span() on `attr` directly
        //         println!(
        //             "Attr: {:?}, Span start line: {}",
        //             attr.meta,
        //             attr.span().start().line
        //         );
        //
        //         if let Meta::NameValue(nv) = &attr.meta {
        //             // if let Lit::Str(lit_str) = nv.value {
        //             //     The string literal contains the content of the /// comment
        //             // println!("  Doc content: {}", lit_str.value());
        //             // }
        //         }
        //     }
        // }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    /// should qwe
    ///
    /// qweqw qw
    #[test]
    fn one_result_case_insensitive() {
        let a = "\
        "
        .to_string();

        assert_eq!(a, "");
    }

    #[cfg(test)]
    mod tests2 {
        #[test]
        fn keka() {
            assert_eq!(2 + 2, 4);
        }
    }
}
