# frozen_string_literal: true

source "https://rubygems.org"

# Specify your gem's dependencies in tree_stump.gemspec
gemspec

gem "rake", "~> 13.0"

gem "rake-compiler"

# Ruby 4.0 compat
# ruby 4.1 HEAD compat requires unreleased changes
if RUBY_VERSION >= "4.1.0"
  # Use rb_sys from GitHub for Ruby 4.1+ (ruby-head), pinned to a specific commit for reproducibility
  gem "rb_sys",
      github: "oxidize-rb/rb-sys",
      ref: "0123456789abcdef0123456789abcdef01234567"
else
  # Use released rb_sys gem for stable Rubies
  gem "rb_sys", "~> 0.9.124"
end
