# Avatars Project Draft

A layer 2 solution for providing avatars on a blockchain platform could work as follows:

1. Users would create and customize their avatars on a layer 2 platform that is built on top of the blockchain. The avatar data would be stored off-chain.
2. Once a user creates an avatar, they would link it to their on-chain identity using a smart contract. The smart contract would store a reference to the off-chain avatar data.
3. When another user interacts with the first user on the blockchain, the blockchain would query the smart contract for the avatar data. The smart contract would return the reference to the off-chain avatar data, which the user's device would then retrieve and display.
4. To prevent fraud, the avatar data would be hashed and stored on-chain. This would ensure that the avatar data cannot be changed without being detected.
5. The layer 2 solution would need to provide scalability and low transaction costs to make it feasible for users to create and customize their avatars.

**Avatars** allow users to have more expressive and personalized on-chain identities, while also maintaining the security and trustlessness of the underlying blockchain platform.
