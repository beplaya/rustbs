# Rust Babysitter Kata  

https://github.com/dnwiebe/Scala_Katas

# Babysitter

A babysitter has the following pay schedule:

* $10/hr to look after children before their bedtime
* $6/hr to house-sit after bedtime
* $8/hr after midnight when work causes her to lose sleep.

Write a method or function that will accept an arrival time, a departure time, and 
a bedtime, and return the babysitter's earnings for the evening in dollars.

A partial hour is always charged as a whole hour; therefore arrival and departure
times will always be integral.  Represent all times as integers on a 12-hour 
clock--not as Dates or as OffsetDateTimes or anything more complicated.
The earliest the babysitter can arrive is 5pm; the latest she can stay is 4am.
Bedtime must never be after midnight.

Dollars in this kata will also always be integral; hence, represent them as 
integers--not as Doubles or BigDecimals or anything more complicated.

The first run at this kata should not include testing or handling for any error
conditions, such as arrival times less than 1 or greater than 12 or bedtimes after
midnight.

