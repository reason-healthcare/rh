library DateTimePrecision version '0.1.0'

define DateMonthPrecisionSameAs: @2024-02 same as @2024-02
define DateMonthPrecisionBeforeDay: @2024-02 same or before @2024-02-15
define DateTimeOffsetCompare: @2024-02-01T10:00:00-05:00 = @2024-02-01T15:00:00Z
define TimePrecisionSameOrAfter: @T10:30 same or after @T10
define DateFromString: ToDate('2024-02-29')
define DateTimeFromStringWithOffset: ToDateTime('2024-02-29T12:15:00-05:00')
