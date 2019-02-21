from parsec import *
import attr

# skip spaces
s = lambda p: p << spaces()

label = (letter() + many(letter() | digit() | string('_'))).parsecmap(
  lambda s: s[0] + "".join(s[1])
)

eq_condition = (
  s(label) + s(string('=')) + s(string('?'))
).parsecmap(
  lambda s: BaseCondition(label=s[0][0], type='eq')
)
base_condition = eq_condition

@generate
def condition():
  conditionA = yield base_condition | condition
  yield spaces()
  join = yield string("AND") | string("OR") | eof()
  if join:
    yield spaces()
    conditionB = yield condition
    return ConditionJoin(
      type=join,
      conditionA=conditionA,
      conditionB=conditionB
    )
  else:
    return conditionA

query = (
  s(string('QUERY'))
  + s(label)
  + s(string('WHERE'))
  + s(condition)
).parsecmap(
  lambda x: Query(
    label=x[0][0][1],
    condition=x[-1]
  )
)
aggregation = many(space())
post = query + aggregation + eof

### AST

@attr.s
class Query(object):
  label = attr.ib()
  condition = attr.ib()

@attr.s
class BaseCondition(object):
  label = attr.ib()
  type = attr.ib()

@attr.s
class ConditionJoin(object):
  type = attr.ib()
  conditionA = attr.ib()
  conditionB = attr.ib()

# AST to JSON

def ast2json(ast):
  return {
    BaseCondition: lambda c: (
      { 'term': { c.label: '?' } }
      if c.type == 'eq' else
      {}
    ),
    ConditionJoin: lambda c: {
      'must' if c.type == 'AND' else 'should': [
        ast2json(c)
        for c in [c.conditionA, c.conditionB]
      ]
    },
    Query: lambda q: {
      'query': {
        'filter': {
          'bool': ast2json(q.condition)
        }
      }
    }
  }[type(ast)](ast)

##
q = query.parse("QUERY foobar WHERE x = ? AND y = ?")
