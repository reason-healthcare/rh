library IntervalBoundaries version '0.1.0'

define ClosedIncludesPoint: Interval[1, 5] includes 5
define OpenExcludesPoint: Interval[1, 5) includes 5
define ProperlyIncludes: Interval[1, 10] properly includes Interval[2, 9]
define OverlapsBefore: Interval[1, 5] overlaps before Interval[5, 10]
define MeetsAfter: Interval[5, 10] meets after Interval[1, 5)
define NullLowBoundary: Interval[null, 5] includes 3
define ExceptAdjacent: Interval[1, 5] except Interval[5, 10]
