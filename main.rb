require 'ffi'

class Condition < FFI::AutoPointer
    def self.release(ptr)
        Binding2.free(ptr)
    end

    def query_str
        Binding2.query_str(self)
    end

    module Binding2
       extend FFI::Library
       ffi_lib 'target/release/libes_dsl.' + FFI::Platform::LIBSUFFIX
       attach_function :free, :cond_free, [Condition], :void
       attach_function :query_str, :cond2query_str, [Condition], :string
    end
end

class BaseCondition < FFI::AutoPointer
    def self.release(ptr)
        Binding2.free(ptr)
    end

    def json_str
        Binding2.json_str(self)
    end

    def +(other)
        Binding2.plus(self, other)
    end

    module Binding2
       extend FFI::Library
       ffi_lib 'target/release/libes_dsl.' + FFI::Platform::LIBSUFFIX
       attach_function :parse, :base_cond_parse, [:string], BaseCondition
       attach_function :free, :base_cond_free, [BaseCondition], :void
       attach_function :json_str, :base_cond2json, [BaseCondition], :string
       attach_function :plus, :base_cond_plus, [BaseCondition, BaseCondition], Condition
    end
end

class Query < FFI::AutoPointer
    def self.release(ptr)
        Binding2.free(ptr)
    end

    def label
        Binding2.label(self)
    end

    def json_str
        Binding2.json_str(self)
    end
    
    module Binding2
       extend FFI::Library
       ffi_lib 'target/release/libes_dsl.' + FFI::Platform::LIBSUFFIX
       attach_function :query_parse, [:string], Query
       attach_function :free, :query_free, [Query], :void
       attach_function :label, :query_label, [Query], :string
       attach_function :json_str, :query2json, [Query], :string
    end
end

cond = BaseCondition::Binding2.parse(
  "foo = ?"
)
cond2 = BaseCondition::Binding2.parse(
  "bar = ?"
)
puts cond.json_str
puts cond2.json_str
puts (cond + cond2).query_str

puts "QUERY foo WHERE #{(cond + cond2).query_str}"
query = Query::Binding2.query_parse(
  "QUERY foo WHERE #{(cond + cond2).query_str}"
)
puts query
puts query.label
puts query.json_str
