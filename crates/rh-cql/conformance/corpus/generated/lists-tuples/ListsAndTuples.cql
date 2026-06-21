library ListsAndTuples version '0.1.0'

define DistinctWithNulls: distinct { 1, 1, null, null, 2 }
define FlattenNested: flatten { { 1, 2 }, { 3 }, null }
define ExceptWithDuplicates: { 1, 1, 2, 3 } except { 1, 3 }
define TupleNestedAccess:
  Tuple { outer: Tuple { inner: 5 } }.outer.inner
define SortTupleList:
  ({ Tuple { id: 'b', score: 2 }, Tuple { id: 'a', score: 1 } }) L
    sort by score
