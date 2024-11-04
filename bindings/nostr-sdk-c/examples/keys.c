#include <stdio.h>
#include <string.h>

#include <nostr_sdk.h>

int main() {
    // Generate keys
    const Keys* keys = keys_generate();
    printf("Keys generated.\n");

    const char* public_key = keys_public_key(keys);
    printf("Public key: %s\n", public_key);

    // Parse keys
    Keys* keysParse = keys_generate();
    keys_parse(keysParse, "nsec1j4c6269y9w0q2er2xjw8sv2ehyrtfxq3jwgdlxj6qfn8z4gjsq5qfvfk99");
    const char* public_key_parse = keys_public_key(keysParse);
    printf("Public key: %s\n", public_key_parse);

    return 0;
}
