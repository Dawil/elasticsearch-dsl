Toy project creating a Query Language for ElasticSearch that isn't some janky json thing.

## Example

```
QUERY
  post_filter
    WHERE
      user_id = ?
      AND (
        collaborator.status = ?
        OR collaborator.status = ?
      )
AGGREGATE
  substate
    WHERE
      HAS substate
      AND (
        user_id = ?
        OR collaborator.id = ?
      )
```

should get turned into

```
query:
  post_filter:
    must:
     - term:
         user_id: ?
     - bool:
         should:
           - term:
               collaborator.status: ? 
           - term:
               collaborator.status: ? 
aggregations:
  substate:
    global: {}
    aggregations:
      substate:
        aggregations:
          filter:
            terms: "substate"
```

## Installation

`pipenv install`

## Developing

Either use `nix-shell` or read that file for a list of dependencies.

## TODO

* AST to string generation
* programatic manipulation of AST (e.g. `ast2str(condition + (condition * condition))`)
