# frozen_string_literal: true

require_relative "tree_stump/version"
require_relative "tree_stump/tree_stump"

module TreeStump
  class Error < StandardError; end

  class QueryError < Error; end
end

