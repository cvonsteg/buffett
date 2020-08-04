# Buffett

A system which calculates metrics and stats on stocks, which may be of interest to an avid stock-picker!

`buffett` takes an input file of stocks with OHLC data, calculates simple metrics based on the stock price, and outputs these to a new file.  A sample OHLC data files is contained in `resources/daily_stocks.txt`.  A sample output file is located in `resources/output_stats.txt`.

**NOTE**: `buffett` is currently a fun project intended to give me more exposure to Rust - the functionality inside is evidence of that!

## Installing

If you wish to use `buffett` locally, simply clone the repo and run the `install.sh` script.

You can then run `buffett <input_file> <output_file>` from anywhere in your terminal

## TODO

1. Clean up file writing mechanism
2. Add unit tests
3. Add more useful calculations
4. Use more suitable dtype for price info, `f64` is overkill
