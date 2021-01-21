# Design of Nomo

Nomo is a blockchain for domain name registration and user identity
verfication. You can think of it as a combination of ENS and Keybase.

Nomo contains a domain name registry, where people can get domain names
they want. Then, a root zone can be edited on-chain based on the owner
information in the registry. This makes it a suitable replacement for the
traditional DNS, similar to how Handshake works. Besides this, we also allow
each domain name owners to maintain a keychain, which allows them to prove
ownership of social accounts, without any centralized authority.

## Philosophy

### Push mode for the root zone

Nomo uses "push mode", which is majorly different from the "pull mode" of
ENS. The ENS contract allows owners of each domain to define custom resolvers,
which services can "pull" to get a full picture of information stored
on-chain. In Nomo, instead, information about each domain are stored
statically. The advantage of doing it this way is that service can now have
confidence in the time needed to generate the root zone, without worrying if any
of the resolver contract might take too long to execute. Nomo's smart contract
pallet allows custom logic to be defined, and the information to be "pushed" to
the zone and the registry.

### Zone and subdomain stays on domain transfer or deletion

When a domain is transferred or deleted, the subdomain registry and the zone
file still stays, governed by the new rule set by the new owner. Not only does
this simplify the design of the registry, but also it enables certain use cases
when the old and the new owner are both smart contracts.

### Public good top-level domains governed by democracy module

For private top-level domains, one can either set their owner to be an
externally-owned account or a smart contract. On the other hand, for certain
top-level domains that are deemed to be public good, their rules are governed by
Substrate's democracy module, and updated together with runtime upgrades.

### Promote ownership and discourage hoarding

A decentralized name service like Nomo, Handshake, ENS or Namecoin all promote
ownership of domain names. On the other hand, we want to encourage that the
domains are put in actual use, and avoid hoarding.

Nomo's top-level domains use similar auction rules to Handshake. However,
instead of auctioning an one-time purchase price, we auction on the renew price
each year. This means that the winner of the auction will be paying the given
price each year in order to keep the top-level domain.

For individual users seeking for an on-chain handle, we provide a single TLD
with the rule of first-come-first-served, and only require an one-time fixed
purchase fee.

### Predictable emission and supply

All purchase fees for domain names go to the decentralized treasury system
on-chain, and the democracy module will decide on how they will be
spent. Compared with burning the fees, this provides a predictable emission, and
allocates some funding for the development of the blockchain.
