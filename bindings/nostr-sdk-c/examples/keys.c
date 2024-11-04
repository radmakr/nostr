#include <stdio.h>
#include <string.h>

#include <nostr_sdk.h>

int main() {
    // Generate keys
    const Keys* keys = keys_generate();
    printf("Public key: %s\n", keys_public_key(keys));

    // Parse keys
    Result_Keys parsed_keys = keys_parse("nsec1j4c6269y9w0q2er2xjw8sv2ehyrtfxq3jwgdlxj6qfn8z4gjsq5qfvfk");
    if (parsed_keys.success) {
        printf("Public key: %s\n", keys_public_key(parsed_keys.output.ok));
    } else {
        printf("Error parsing secret key: %s\n", parsed_keys.output.err);
    }

    return 0;
}
