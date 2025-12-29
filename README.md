# TreeStump
[![RSpec](https://github.com/joker1007/tree_stump/actions/workflows/rspec.yml/badge.svg)](https://github.com/joker1007/tree_stump/actions/workflows/rspec.yml)

[tree-sitter](https://github.com/tree-sitter/tree-sitter) binding for MRI Ruby written by Rust.

## Installation

Install the gem and add to the application's Gemfile by executing:

    $ bundle add tree_stump

If bundler is not being used to manage dependencies, install the gem by executing:

    $ gem install tree_stump

## Platform Compatibility

tree_stump is a Rust native extension using [magnus](https://github.com/matsadler/magnus) and [rb-sys](https://github.com/oxidize-rb/rb-sys) for Ruby bindings.

| Platform | Supported | Notes |
|----------|-----------|-------|
| **MRI Ruby** | ✅ Yes | Full support (3.1+) |
| **JRuby** | ❌ No | JRuby runs on the JVM and cannot load native `.so` extensions |
| **TruffleRuby** | ❌ No | magnus/rb-sys are incompatible with TruffleRuby's C API emulation |

<details>
    <section>Why JRuby doesn't work</section>

JRuby runs on the Java Virtual Machine and cannot load native C/Rust extensions (`.so` files). The `rb-sys` build system fails during `extconf.rb` because JRuby doesn't provide the platform detection APIs that native extensions require.

**Alternative for JRuby**: Use [java-tree-sitter](https://github.com/tree-sitter/java-tree-sitter) / [jtreesitter](https://central.sonatype.com/artifact/io.github.tree-sitter/jtreesitter) directly, or use [TreeHaver](https://github.com/kettle-rb/tree_haver)'s Java backend which wraps it.

</details>

<details>
    <section>Why TruffleRuby doesn't work</section>

TruffleRuby implements Ruby on GraalVM, not as a C program. While it has a C API emulation layer (via Sulong/LLVM), this layer doesn't expose all of MRI's internal structures that `magnus` requires:

- Missing C API functions: `rb_bug`, `rb_warning`, `rb_gc_writebarrier`, `rb_data_define`, etc.
- Missing struct fields: `RBasic.flags`, `RBasic.klass`
- Missing constants: `ruby_rvalue_flags`, `RUBY_ENC_CODERANGE_MASK`

These are MRI implementation details that TruffleRuby doesn't need (and can't easily provide) because its objects are implemented completely differently.

**Alternative for TruffleRuby**: Use [TreeHaver](https://github.com/kettle-rb/tree_haver) with non-tree-sitter backends (prism, psych, citrus) for parsing needs.

</details>

## Usage

```ruby
require "tree_stump"
require "rouge"

TreeStump.register_lang("ruby", "./libtree-sitter-ruby.so")

parser = TreeStump::Parser.new
parser.set_language("ruby")

source = File.read("./sample.rb")

formatter = Rouge::Formatters::Terminal256.new
lexer = Rouge::Lexers::Ruby.new

puts "== Source =="
puts formatter.format(lexer.lex(source))

puts "\n"

puts "==  Tree  =="
tree = parser.parse(source)

puts tree.root_node.to_sexp
```

```
(program (class name: (constant) superclass: (superclass (constant)) body: (body_statement (call method: (identifier) arguments: (argument_list (simple_symbol) (pair key: (hash_key_symbol) value: (true)))) (singleton_method object: (self) name: (identifier) parameters: (method_parameters (identifier)) body: (body_statement (call receiver: (constant) method: (identifier) arguments: (argument_list (pair key: (hash_key_symbol) value: (identifier)))))))) (call receiver: (constant) method: (identifier) arguments: (argument_list (string (string_content)))))
```

## Development

### Requirements

- Rust Toolchain
- MRI Ruby (not JRuby, or TruffleRuby)
- tree-sitter-ruby

1. Download source of tree-sitter-ruby from [GitHub Repository](https://github.com/tree-sitter/tree-sitter-ruby).
2. Extract tree-sitter-ruby source
3. mv tree-sitter-ruby-v{version_num} to tree_stump/tree-sitter-ruby
4. Execute `make` in tree-sitter-ruby directory
5. Execute `bundle exec rake compile`

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/joker1007/tree_stump.
