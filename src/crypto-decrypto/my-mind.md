这题加解密，其实我是有思路的。

没做是因为我严格计时，没有做完。之前在椭圆曲线那里的点的非压缩和压缩格式那里查资料浪费了不少时间。


这个题目的解答思路:

1. 首先通过Rust把秘钥拿到，加密的秘钥就是通过加密逻辑拿到derivekey和5031a85957e7fdb47d6ece9d95adbc36这个salt，秘钥就是这个二元组(derivedKey, salt)
2. 通过秘钥和对应的aes-128-ctr解密算法对ciphertext做解密。


实在是没有时间了，不好意思。

