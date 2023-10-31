# Interval Info
A CLI tool for analyzing properties of just musical intervals  
To find just intervals, you may use the [interval-finder](https://github.com/Artour64/interval-finder) tool.  

## Arguments
Running without arguments displays a message telling you to input an interval.  
The last argument is the input interval to analyze.  
The format should be as two positive integers separated by a forward slash e.g. `3/2`.  
Intervals not written in simplest form will automatically be simplified.  
Intervals that have the denominator smaller than the numerator will automatically be flipped.  
Invalid arguments are ignored.  

### Options
-h --help  
 Display the help message.  

-1 --scale-start-1  
 Start scale tonic note count at 1 instead of 0.  
