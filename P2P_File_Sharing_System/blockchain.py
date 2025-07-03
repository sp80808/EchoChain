from substrateinterface import SubstrateInterface, Keypair
from substrateinterface.exceptions import SubstrateRequestException

NODE_URL = "ws://127.0.0.1:9944"  # Assuming a local Substrate node


def get_substrate_instance():
    try:
        return SubstrateInterface(url=NODE_URL)
    except ConnectionRefusedError:
        print(
            f"Connection to Substrate node at {NODE_URL} refused. Is the node running?"
        )
        return None
    except Exception as e:
        print(f"Error connecting to Substrate node: {e}")
        return None


def register_file_on_chain(file_hash, owner_mnemonic, metadata):
    substrate = get_substrate_instance()
    if not substrate:
        return False

    try:
        # Derive keypair from mnemonic
        keypair = Keypair.create_from_uri(owner_mnemonic)

        # Example: Call a hypothetical 'FileStorage' pallet's 'registerFile' extrinsic
        # This assumes you have a custom pallet with this functionality.
        # You'll need to replace 'FileStorage' and 'registerFile' with your actual pallet and extrinsic names.
        # The metadata should be structured according to your pallet's definition.
        call = substrate.compose_call(
            call_module="FileStorage",
            call_function="registerFile",
            call_params={
                "file_hash": file_hash,
                "metadata": metadata,
            },
        )

        # Create and sign the extrinsic
        extrinsic = substrate.create_signed_extrinsic(
            call=call, keypair=keypair, era={"period": 64}, tip=0
        )

        # Submit the extrinsic
        receipt = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)

        print(
            f"File {file_hash} registered on chain. Extrinsic hash: {receipt.extrinsic_hash}"
        )
        return True
    except SubstrateRequestException as e:
        print(f"Substrate request error during registration: {e}")
        return False
    except Exception as e:
        print(f"Error registering file on chain: {e}")
        return False


def verify_file_on_chain(file_hash, owner_address=None):
    substrate = get_substrate_instance()
    if not substrate:
        return False

    try:
        # Example: Query a hypothetical 'FileStorage' pallet's 'Files' storage map
        # This assumes your pallet stores file information accessible via a storage map.
        # You'll need to replace 'FileStorage' and 'Files' with your actual pallet and storage map names.
        result = substrate.query(
            module="FileStorage", storage_function="Files", params=[file_hash]
        )

        file_info = result.value
        if file_info:
            print(f"File {file_hash} found on chain. Info: {file_info}")
            if owner_address:
                # Check if the owner matches the provided address
                if str(file_info["owner"]) == owner_address:
                    print(f"File {file_hash} is owned by {owner_address}.")
                    return True
                else:
                    print(
                        f"File {file_hash} is not owned by {owner_address}. Owner: {file_info['owner']}"
                    )
                    return False
            return True
        else:
            print(f"File {file_hash} not found on chain.")
            return False
    except SubstrateRequestException as e:
        print(f"Substrate request error during verification: {e}")
        return False
    except Exception as e:
        print(f"Error verifying file on chain: {e}")
        return False
