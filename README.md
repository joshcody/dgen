# dgen


# WIP

Data generator that produces JSON records of configurable attributes

Supported types: 
- ip address
- pop
- path (random words)


The generator replaces tokens in a config file. See `dgen.toml` for an example


--
Custom POP list 
POPs are loaded from an embedded text file containing airport codes and locations from around the world. 
If you'd rather provide a custom list, you can add either `--from-list` or `--from-file` with values you'd like to choose from

### from list
dgen --pop --from-list popA,popB,popC

### from file
dgen --pop --from-file pops.txt

### from list in the config file
The config file will automatically select one of any array provided. Change the value of `pop` from the `<pop>` keywoord to a list of options
```
[fields]
client_ip = "<ipaddress>"
server_ip = "<ipaddress>"
pop = ["popA","popB","popC","popZ"]
method = ["GET","POST","OPTION"]
path = "/<word>/<word>/<word>" 
```


TODO: 

- Create / Improve documentation
- Add uniquness constraints for each value to collapse the surface area for data testing
- Handle proper class C CIDR notations for randomizing IP addresses
- Add IPv6 address generation
- Expand the generatable pool of fields
