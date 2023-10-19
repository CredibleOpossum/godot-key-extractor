# Godot key extractor
Extracts the encryption key of Godot projects

# Why
Many developers want to keep their stuff safe. This project might seem like an attack on them. But, the main goal of this project is to make it easy for developers, like me, to see if their project can withstand automated tools.

# So what should I do?
Probably nothing, it's not the end of the world or anything. If you are interested in protecting your assets I would recommend modifying the Godot source code and change how the key is stored, you could XOR or even just reverse the key.

The file you would have to modify is [this one](https://github.com/godotengine/godot/blob/master/core/io/file_access_encrypted.cpp).
