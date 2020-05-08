import { ElGamal } from "apollo-crypto";

const elgamal = ElGamal.new();
const cipher = elgamal.encrypt(true);
const [c1, c2] = cipher.split(',');
console.log(c1);
console.log(c2);
console.log(elgamal.decrypt(c1, c2));
