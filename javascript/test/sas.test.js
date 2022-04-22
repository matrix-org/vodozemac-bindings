const { Sas } = require('../pkg/vodozemac.js');

const EXTRA_INFO = "extra_info";
const MESSAGE = "Test message"

describe('Sas', function() {
    it('should be created successfully', function() {
        let alice = new Sas();

        expect(alice.public_key).not.toBe("");
    });

    it('should allow us to establish a shared secret', function() {
        let alice = new Sas();
        let bob = new Sas();
        const bob_public_key = bob.public_key;

        bob = bob.diffie_hellman(alice.public_key);
        alice = alice.diffie_hellman(bob_public_key);
    });

    it('should allow us to generate common short auth strings', function() {
        let alice = new Sas();
        let bob = new Sas();
        const bob_public_key = bob.public_key;

        bob = bob.diffie_hellman(alice.public_key);
        alice = alice.diffie_hellman(bob_public_key);

        const alice_bytes = alice.bytes(EXTRA_INFO);
        const bob_bytes = bob.bytes(EXTRA_INFO);

        expect(alice_bytes.emoji_indices).toEqual(bob_bytes.emoji_indices);
        expect(alice_bytes.decimals).toEqual(bob_bytes.decimals);
    });

    it('should allow us to generate a message authentication code', function() {
        let alice = new Sas();
        let bob = new Sas();
        const bob_public_key = bob.public_key;

        bob = bob.diffie_hellman(alice.public_key);
        alice = alice.diffie_hellman(bob_public_key);

        const alice_mac = alice.calculate_mac(MESSAGE, EXTRA_INFO);
        const bob_mac = bob.calculate_mac(MESSAGE, EXTRA_INFO);

        expect(() => alice.verify_mac("", EXTRA_INFO, bob_mac)).toThrow();
        alice.verify_mac(MESSAGE, EXTRA_INFO, bob_mac);
        bob.verify_mac(MESSAGE, EXTRA_INFO, alice_mac);
        expect(alice_mac).toEqual(bob_mac);
    });
});
