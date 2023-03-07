# Kotlin Tools
This inner repository includes artifacts for assistance in communicating with the group-member-approval-smart-contract 
in JVM environments.  This directory is used when new contract releases occur to build a kotlin client for contract
communication, and to build a set of dev tools that allow for easy storage and instantiation of the contract during
integration testing.

## Client

### Download
To fetch the `client` via Maven, use the following dependency qualifier: 
```text
tech.figure.approval.member.group:member-approval-client:{latest-version}
```

### Usage
The client should be instantiated with a `PbClient` instance, which allows message broadcasting to Provenance Blockchain
instances, when necessary.  Instantiate the client with the following block:

```kotlin
import io.provenance.client.grpc.GasEstimationMethod
import io.provenance.client.grpc.PbClient
import java.net.URI
import tech.figure.approval.member.group.client.client.GroupMemberContractClient
import tech.figure.approval.member.group.client.util.GroupMemberContractAddressResolver

fun getPbClient(): PbClient = PbClient(
    chainId = "chain-local",
    channelUri = URI.create("grpc://some-provenance-uri:9090"),
    gasEstimationMethod = GasEstimationMethod.MSG_FEE_CALCULATION,
)

fun getClient(): GroupMemberContractClient = GroupMemberContractClient(
    pbClient = getPbClient(),
    addressResolver = GroupMemberContractAddressResolver.FromName(name = "mycontractname.sc.pb"),
)
```

After creating a `GroupMemberContractClient`, the core usage is to either execute the contract to add attributes to
signing members, or to view the contract state.

#### Adding a member approval programmatically:

```kotlin
import io.provenance.client.wallet.WalletSigner
import io.provenance.hdwallet.bip39.MnemonicWords
import io.provenance.hdwallet.wallet.Wallet
import tech.figure.approval.member.group.client.client.GroupMemberContractClient
import tech.figure.approval.member.group.client.dto.executemsg.ApproveGroupMembershipResponse
import tech.figure.approval.member.group.client.dto.executemsg.ExecuteApproveGroupMembership

fun generateSigner(providedMnemonicString: String? = null): WalletSigner = Wallet.fromMnemonic(
    hrp = "tp",
    passphrase = "",
    // IMPORTANT: ALWAYS KEEP YOUR MNEMONICS SECRET
    mnemonicWords = providedMnemonicString?.let(MnemonicWords::of) ?: MnemonicWords.generate(strength = 256),
    testnet = true,
)["m/44'/1'/0'/0/0'"].let(::WalletSigner)

fun addMemberApproval(client: GroupMemberContractClient, forGroup: Long): ApproveGroupMembershipResponse {
    val response = client.executeGroupMemberApproval(
        executeMsg = ExecuteApproveGroupMembership(groupId = forGroup),
        signer = generateSigner(),
    )
    check(response.groupId == forGroup) { "Group $forGroup was expected to be approved. Instead: ${response.groupId}" }
    return response
}
```

#### Querying contract state
```kotlin
import tech.figure.approval.member.group.client.client.GroupMemberContractClient
import tech.figure.approval.member.group.client.dto.querymsg.GroupMemberContractState

fun fetchContractState(
    client: GroupMemberContractClient,
    expectedAdmin: String,
): GroupMemberContractState = client.queryContractState().also { state ->
    check(state.admin == expectedAdmin) { "The admin was anticipated to be $expectedAdmin, but was actually ${state.admin}" }
}
```

## Dev Tools

### Download
To fetch the `devtools` via Maven, use the following dependency qualifier:
```text
tech.figure.approval.member.group:member-approval-devtools:{latest-version}
```

### Usage
The devtools include an implementation of the `GroupMemberApprovalClient` that extends its functionality to include
the ability to create the smart contract programmatically on a specific chain instance.  Create one in the same way you
would create a `GroupMemberApprovalClient`:

```kotlin
import io.provenance.client.grpc.GasEstimationMethod
import io.provenance.client.grpc.PbClient
import java.net.URI
import tech.figure.approval.member.group.client.client.GroupMemberContractClient
import tech.figure.approval.member.group.client.util.GroupMemberContractAddressResolver
import tech.figure.approval.member.group.devtools.client.LocalGroupMemberContractClient

fun getPbClient(): PbClient = PbClient(
    chainId = "chain-local",
    channelUri = URI.create("grpc://some-provenance-uri:9090"),
    gasEstimationMethod = GasEstimationMethod.MSG_FEE_CALCULATION,
)

fun getClient(): GroupMemberContractClient = LocalGroupMemberContractClient(
    pbClient = getPbClient(),
    addressResolver = GroupMemberContractAddressResolver.FromName(name = "mycontractname.sc.pb"),
)
```

The `instantiateContract` includes the option for a manual contract instance to be supplied via filesystem, project
resources, or to automatically download it from the GitHub repository by version tag.  To use this functionality:

```kotlin
import io.provenance.client.wallet.WalletSigner
import io.provenance.hdwallet.bip39.MnemonicWords
import io.provenance.hdwallet.wallet.Wallet
import tech.figure.approval.member.group.devtools.client.LocalGroupMemberContractClient
import tech.figure.approval.member.group.devtools.client.model.GroupMemberContractInstantiationMode
import tech.figure.approval.member.group.devtools.client.model.GroupMemberContractWasmLocation
import tech.figure.approval.member.group.devtools.feign.models.instantiatemsg.InstantiateGroupMemberContract
import tech.figure.approval.member.group.devtools.feign.models.instantiatemsg.InstantiateGroupMemberContractResponse

fun generateSigner(providedMnemonicString: String? = null): WalletSigner = Wallet.fromMnemonic(
    hrp = "tp",
    passphrase = "",
    // IMPORTANT: ALWAYS KEEP YOUR MNEMONICS SECRET
    mnemonicWords = providedMnemonicString?.let(MnemonicWords::of) ?: MnemonicWords.generate(strength = 256),
    testnet = true,
)["m/44'/1'/0'/0/0'"].let(::WalletSigner)

fun instantiateContract(client: LocalGroupMemberContractClient): InstantiateGroupMemberContractResponse {
    val response = client.instantiateContract(
        instantiateMsg = InstantiateGroupMemberContract(
            contractName = "My awesome contract",
            attributeName = "awesomecontract.sc.pb",
            bindAttributeName = true,
        ),
        admin = generateSigner(providedMnemonicString = "some mnemonic words blah blah blah blah blah blah blah"),
        // Note: The code below is not valid Kotlin code because it duplicates arguments.  It is intended to showcase
        // all possible input parameters for better insights into how they work.
        // This example would instantiate a group-member-approval-smart contract already stored on chain with code id #10004
        instantiationMode = GroupMemberContractInstantiationMode.InstantiateOnly(codeId = 10004),
        // Or to both store the contract instance and instantiate it on chain, use one of the following:
        instantiationMode = GroupMemberContractInstantiationMode.StoreAndInstantiate(
            // This would download the contract from a GitHub release and store it for you.  This is the recommended path
            // because it ensures the contract has passed all necessary repository validation
            wasmLocation = GroupMemberContractWasmLocation.GitHub(contractReleaseTag = "v1.0.1"),
            // Or to store the contract from a path anywhere on the local machine, use the following:
            wasmLocation = GroupMemberContractWasmLocation.LocalFile.AbsolutePath(absoluteFilePath = "~/mycomputer/wasms/contract.wasm"),
            // Or to store the contract from a path in the current project resources, use the following:
            wasmLocation = GroupMemberContractWasmLocation.LocalFile.ProjectResource(resourcePath = "wasms/contract.wasm"),
        )
    )
    // For some reason it's incredibly important that this newly-added code id is the fifteenth!
    check (response.storedCodeId == 15L) { "Unexpected code id in response: ${response.storedCodeId}" }
    return response
}
```
