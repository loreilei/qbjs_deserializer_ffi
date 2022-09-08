## About this crate

This crate expose the [qbjs_deserializer crate](https://github.com/loreilei/qbjs_deserializer) through a C-style ffi.

## Example usage

Ensure the generated `include/qbjs_deserializer_ffi/qbjs_deserializer.h` file is in your project include path.
Also ensure you're linking to the generated dynamic library file (OS dependent).

Once these requirements are met, you can simply:

```c

#include <qbjs_deserializer_ffi/qbjs_deserializer.h>

// In your code...
// 1. Use your favorite file reading API to read the content of the file you want
// to deserialize and get it as an array of uint8_t, also knowing the size of the array
uint8_t* fileBinaryContent = // ...
uint64_t fileBinaryContentSize = // ...

char* deserializedFileContent = deserialize_to_json_string(
        fileBinaryContent, fileBinaryContentSize);

if (!deserializedFileContent)
{
    //1.a. Oops, a null pointer was returned, it means the API failed to deserialize the provided content. Handle the error.
}

// 2. Give this string to your favorite JSON parsing API.

// 3. Once your JSON data is parsed and the string isn't required anymore, free the returned value via the ffi (Rust need to deallocate this string on its side)
free_deserialized_string(deserialized_qbjs);

// 4. Work on your data...

```




