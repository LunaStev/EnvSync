# EnvSync

EnvSync is an extension of Rust's envie library, providing functionality to synchronize environment variables between memory and the system environment. This library helps easily synchronize environment variable values between the .env file and system environment variables.

## Features
* `reload()`: Reload the .env file and synchronize with system environment variables.
* `get_system_env()`: Retrieve a value from system environment variables.
* `get_mem_env()`: Retrieve a value from memory-stored environment variables.
* ```set()`: Set environment variable values in both memory and system environment.
* `remove()`: Remove environment variables from both memory and system environment.
* `export_all_to_system()`: Export all memory-stored environment variables to system environment variables.

## Installation
Add envsync to your Cargo.toml to use the library:

```toml
[dependencies]
envsync = "0.1.0"
```

## Usage
### Initialize EnvSync
To create an EnvSync instance, load and synchronize the .env file.

```
use envsync::EnvSync;

fn main() -> Result<(), String> {
    let mut env_sync = EnvSync::new()?;
    Ok(())
}
```
### Reload and Synchronize
Call reload() to reload the .env file and synchronize with system environment variables.

```
env_sync.reload()?;
```

### Set Environment Variables
Use set() to set values in both memory and system environment variables.
```
env_sync.set("MY_KEY", "my_value")?;
```

### Retrieve Environment Variables
Use get_mem_env() and get_system_env() to get values from memory and system environment variables.

```
let value_mem = env_sync.get_mem_env("MY_KEY");
let value_system = env_sync.get_system_env("MY_KEY");
```

### Remove Environment Variables
Use remove() to remove environment variables from both memory and system environment variables.

```
env_sync.remove("MY_KEY")?;
```

### Export All Environment Variables to System
Use export_all_to_system() to export all memory-stored environment variables to system environment variables.

```
env_sync.export_all_to_system()?;
```

## Example

```
use envsync::EnvSync;
use std::env;

fn main() -> Result<(), String> {
let mut env_sync = EnvSync::new()?;

    // Set environment variable
    env_sync.set("MY_KEY", "my_value")?;
    
    // Retrieve values
    println!("MY_KEY (Memory): {:?}", env_sync.get_mem_env("MY_KEY"));
    println!("MY_KEY (System): {:?}", env_sync.get_system_env("MY_KEY"));
    
    // Remove environment variable
    env_sync.remove("MY_KEY")?;
    
    // Export all environment variables to system
    env_sync.export_all_to_system()?;
    
    Ok(())
}
```

## License
This project is licensed under the MPL License. See the LICENSE file for more details.

