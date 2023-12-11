use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Ident, ItemFn, Lit, NestedMeta};

// CM: Note:  I got these macros from here:
// https://github.com/AxlLind/AdventOfCode2022

#[proc_macro_attribute]
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_path = match &parse_macro_input!(args as AttributeArgs)[..] {
        [NestedMeta::Lit(Lit::Int(day))] => format!("../../inputs/{}.in", day.token()),
        _ => panic!("Expected one integer argument"),
    };

    let mut aoc_solution = parse_macro_input!(input as ItemFn);
    aoc_solution.sig.ident = Ident::new("aoc_solution", aoc_solution.sig.ident.span());

    let tokens = quote! {
        use clap::Parser;
        const INPUT: &str = include_str!(#input_path);

        #[derive(Parser)]
        struct Cli {
            #[arg(short, long, default_value_t = false)]
            perf: bool
        }

        #aoc_solution

        fn main() {
            let args = Cli::parse();
            let mut tot : f64 = 0.0;
            let mut out : usize = 0;
            let mut num_tries = 1;
            if args.perf == true {
                num_tries = 1000
            }
            for part in 0..=1 {
                for _ in 0..num_tries {
                    let now = ::std::time::Instant::now();
                    out = aoc_solution(part, INPUT.trim_end());
                    let elapsed = now.elapsed();
                    tot += elapsed.as_nanos() as f64;
                }
                tot = tot / num_tries as f64;
                if !args.perf {
                    println!("\nPart {}: {}", part + 1, out);
                    if tot < 1000.0 {
                        println!("Time: {:.1}ns", tot as usize);
                    }
                    else if tot < 1000000.0 {
                        println!("Time: {:.1}μs", (tot / 1000.0));
                    }
                    else {
                        println!("Time: {:.1}ms", (tot / 1000000.0));
                    }
                }
                else {
                    if part > 0 {
                        print!(",");
                    }
                    print!("{}", tot as usize);
                }
            }
            //if elapsed.as_millis() > 0 {
            //  println!("Time: {}ms", elapsed.as_millis());
            //} else {
            //}
        }
    };
    TokenStream::from(tokens)
}
