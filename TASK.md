# Task

Implement a pallet (=module of a Substrate based blockchain) that pays a constant reward to the block author.

1. Add a Storage item for the reward amount
2. Add an extrinsic call to set the reward amount in storage
3. On each block, mint new coins and pay them

## Resources

* [Substrate Documentation](https://docs.substrate.io)
* [Substrate Rust Documentation](https://paritytech.github.io/substrate/)
* [FRAME & Pallets](https://docs.substrate.io/reference/frame-pallets/)
* [Pallet builder basics](https://docs.substrate.io/tutorials/collectibles-workshop/00-intro-runtime-pallets/)
* [Example Pallet with a lot of FRAME functionalities](https://github.com/paritytech/substrate/tree/master/frame/examples/basic)
* [Pallet that tracks block authors](https://github.com/paritytech/substrate/blob/master/frame/authorship/src/lib.rs)
* [Pallet where KILT does something similar](https://github.com/KILTprotocol/kilt-node/blob/develop/pallets/parachain-staking/src/lib.rs)
