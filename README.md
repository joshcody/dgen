# dgen


# WIP

Data generator that produces JSON records of configurable attributes

Supported types: 
- ip address
- pop
- path (random words)


The generator replaces tokens in a config file. See `dgen.toml` for an example


TODO: 

- Create / Improve documentation
- Add uniquness constraints for each value to collapse the surface area for data testing
- Handle proper class C CIDR notations for randomizing IP addresses
- Add IPv6 address generation
- Expand the generatable pool of fields
