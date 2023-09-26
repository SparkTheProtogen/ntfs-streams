# ntfs-streams

This is a program for writing data into NTFS alternate data streams, currently can only read / write files into streams, more features to come.

The possible modes are:
+ f2s (file to stream): ntfs-streams -m f2s -f file_to_store -s targetfile:targetstream
+ s2f (stream to file): ntfs-streams -m s2f -f file_to_extract -s targetfile:targetstream
+ ms2f mass stream to file, contents of --stream irrelevant
**When using the "ms2f" option, put _only stream names_ in index.txt** 

Mass file to stfeam can be done with a batch for loop
`for /r %cd% %i in (*.WAV) do ntfsstreams -m f2s -f %i -s targetfile:%i`
With above method file names CANNOT have spaces in them



Help screen:
Usage: ntfsstreams --mode <MODE> --stream <STREAM> --file <FILE>

Options:
  -m, --mode <MODE>
  -s, --stream <STREAM>
  -f, --file <FILE>
  -h, --help             Print help (see more with '--help')
  -V, --version          Print version
