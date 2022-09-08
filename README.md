## About this crate

This crate expose the [qbjs_deserializer crate](https://github.com/loreilei/qbjs_deserializer) through a C++-style ffi.

## Example usage

Ensure the generated `include/qbjs_deserializer_ffi/qbjs_deserializer.hpp` file is in your project include path.
Also ensure you're linking to the generated dynamic library file (OS dependent).

Once these requirements are met, you simply:

```cpp

#include <qbjs_deserializer_ffi/qbjs_deserializer.hpp>

// In your code...
std::vector<uint8_t> fileBinaryContent = // Use your favorite file reading API to read the content of the file you want to deserialize and get it as a vector of uint8_t

auto deserializedFileContent = qbjs_deserializer_ffi::deserialize_to_json_string(
        fileBinaryContent.data(), fileBinaryContent.size());

if (!deserializedFileContent)
{
    // A null pointer was returned, it means the API failed to deserialize the provided content. Handle the error.
}

// Optional: build a std::string from the c-style string for convenience
std::string deserializedQbjsStr(deserialized_qbjs);

// Give this string to your favorite JSON parsing API.

// Once your JSON data is parsed and the string isn't required anymore, free the returned value via the ffi (Rust need to deallocated this string on its side)
qbjs_deserializer_ffi::free_deserialized_string(deserialized_qbjs);

// Work on your data...

```




